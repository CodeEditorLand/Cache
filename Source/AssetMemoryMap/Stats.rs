//! Snapshot of asset-cache stats for diagnostics.

use crate::AssetMemoryMap::{CacheStats, Map};

/// Capture a diagnostic snapshot of the asset cache.
///
/// Iterates the entire map to collect entry count, byte totals, and
/// brotli-sibling statistics.
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
