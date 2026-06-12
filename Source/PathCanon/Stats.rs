use crate::PathCanon::{Cache::CACHE, CacheStats};

/// Captures a diagnostic snapshot of the canonical-path cache.
///
/// Iterates the process-wide cache to collect the current entry count
/// and weighted-size estimate.
///
/// # Returns
///
/// A [`CacheStats::Struct`] containing:
/// - `Entries` — number of entries currently held in the cache.
/// - `WeightedSize` — total weighted size (byte-estimate) of cached entries.
pub fn Fn() -> CacheStats::Struct {
	CacheStats::Struct {
		Entries:CACHE.entry_count() as usize,

		WeightedSize:CACHE.weighted_size() as usize,
	}
}
