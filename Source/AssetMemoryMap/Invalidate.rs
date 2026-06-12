use std::{path::Path, sync::Arc};

use crate::AssetMemoryMap::{Entry, Map};

/// Drops a single cached entry, returning it if it existed.
///
/// Useful for hot-reload during development when the bundler rewrites
/// a chunk; the caller can inspect or discard the old entry.
///
/// # Parameters
///
/// * `Path` — the file path whose cached entry should be removed.
///
/// # Returns
///
/// `Some(Arc<Entry::Struct>)` if the entry existed, `None` otherwise.
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
///
/// use land_cache::AssetMemoryMap::Invalidate;
///
/// let old = Invalidate::Fn(Path::new("/path/to/chunk.js"));
/// ```
pub fn Fn(Path:&Path) -> Option<Arc<Entry::Struct>> { Map::Fn().remove(Path).map(|(_, V)| V) }
