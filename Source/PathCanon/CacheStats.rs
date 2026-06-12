//! Diagnostic snapshot of the canonical-path cache, returned by
//! [`super::Stats::Fn`].

/// Occupancy snapshot returned by [`super::Stats::Fn`]. Records entry
/// count and weighted-size estimate from the moka cache.
#[derive(Debug, Clone, Copy)]
pub struct Struct {
	/// Number of entries currently held in the cache.
	pub Entries:usize,

	/// Total weighted size (byte-estimate) of all cached entries.
	pub WeightedSize:usize,
}
