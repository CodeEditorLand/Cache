use crate::AssetMemoryMap::{CacheStats, Map};

/// Captures a diagnostic snapshot of the asset cache.
///
/// Iterates the entire map to collect the entry count, total
/// uncompressed bytes, and brotli-sibling statistics.
///
/// # Returns
///
/// A [`CacheStats::Struct`] containing:
/// - `Entries` — number of cached entries.
/// - `BrotliEntries` — entries that have a pre-compressed brotli sibling.
/// - `Bytes` — total uncompressed bytes across all entries.
/// - `BrotliBytes` — total pre-compressed brotli bytes.
pub fn Fn() -> CacheStats::Struct {
	let mut Bytes = 0usize;

	let mut Entries = 0usize;

	let mut BrotliEntries = 0usize;

	let mut BrotliBytes = 0usize;

	for Reference in Map::Fn().iter() {
		Entries += 1;

		Bytes += Reference.value().Length;

		if let Some(BLength) = Reference.value().BrotliLength() {
			BrotliEntries += 1;

			BrotliBytes += BLength;
		}
	}

	CacheStats::Struct { Entries, BrotliEntries, Bytes, BrotliBytes }
}
