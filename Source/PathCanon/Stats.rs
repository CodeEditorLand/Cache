//! Captures a diagnostic snapshot of the canonical-path cache.

use crate::PathCanon::{Cache::CACHE, CacheStats};

/// Captures a diagnostic snapshot of the canonical-path cache.
///
/// Returns the current entry count and weighted-size estimate.
pub fn Fn() -> CacheStats::Struct {
	CacheStats::Struct {
		Entries:CACHE.entry_count() as usize,

		WeightedSize:CACHE.weighted_size() as usize,
	}
}
