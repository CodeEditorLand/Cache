use std::{
	path::{Path, PathBuf},
	sync::Arc,
};

use memmap2::Mmap;

use crate::AssetMemoryMap::{Entry, Map, MimeFromExtension};

/// Loads the file at `Path` into the asset cache, or returns the
/// existing cached entry.
///
/// On a cache miss, opens the file, memory-maps it, computes metadata
/// (MIME type, ETag, content length), and attempts to locate an
/// optional brotli-precompressed sibling (`<path>.br`). The resulting
/// [`Entry::Struct`] is inserted into the process-wide map and returned
/// as an [`Arc`].
///
/// # Parameters
///
/// * `Path` — the file path to load into the cache.
///
/// # Returns
///
/// `Ok(Arc<Entry::Struct>)` — the cached entry, either existing or
/// freshly loaded.
///
/// # Errors
///
/// Returns `Err` only if the file cannot be opened or memory-mapped;
/// missing brotli siblings are silently ignored (best-effort
/// optimisation).
///
/// # Safety
///
/// The caller agrees the file is not truncated underneath the mapping
/// for its lifetime. The bundle directory is read-only at runtime;
/// mutations happen at build time and require a binary restart.
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use land_cache::AssetMemoryMap::LoadOrInsert;
///
/// let entry = LoadOrInsert::Fn(Path::new("/path/to/asset.js")).unwrap();
/// ```
pub fn Fn(Path:&Path) -> std::io::Result<Arc<Entry::Struct>> {
	if let Some(Existing) = Map::Fn().get(Path) {
		return Ok(Existing.clone());
	}

	let File = std::fs::File::open(Path)?;

	let Metadata = File.metadata()?;

	let Length = Metadata.len() as usize;

	let ModifiedMs = Metadata
		.modified()
		.ok()
		.and_then(|Time| Time.duration_since(std::time::UNIX_EPOCH).ok())
		.map(|Duration| Duration.as_millis() as u64)
		.unwrap_or(0);

	let ETag = format!("W/\"{:x}-{:x}\"", ModifiedMs, Length);

	// SAFETY: caller agrees the file is not truncated underneath us
	// for the lifetime of the MemoryMap. The bundle directory is
	// read-only at runtime; mutations happen at build time and require
	// a binary restart.
	let Mapping = unsafe { Mmap::map(&File)? };

	let BrotliPath = {
		let mut B = Path.as_os_str().to_owned();

		B.push(".br");

		PathBuf::from(B)
	};

	let Brotli = std::fs::File::open(&BrotliPath)
		.ok()
		.and_then(|F| unsafe { Mmap::map(&F).ok() });

	let Mime = MimeFromExtension::Fn(Path);

	let MarkerEntry = Arc::new(Entry::Struct { Mapping, Mime, Length, Brotli, ETag });

	log::debug!(
		target:"asset-cache",

		"mmap insert path={} bytes={} brotli={}",

		Path.display(),

		Length,

		MarkerEntry.Brotli.is_some()
	);

	Map::Fn().insert(Path.to_path_buf(), MarkerEntry.clone());

	Ok(MarkerEntry)
}
