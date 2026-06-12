/// Occupancy snapshot returned by [`super::Stats::Fn`].
///
/// Records the entry count and weighted-size estimate from the moka
/// cache at the moment the snapshot was taken.
///
/// # Examples
///
/// ```rust,no_run
/// use land_cache::PathCanon::CacheStats;
///
/// let stats = CacheStats::Struct {
///     Entries: 42,
///     WeightedSize: 65536,
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Struct {
	/// Number of entries currently held in the cache.
	pub Entries:usize,

	/// Total weighted size (byte-estimate) of all cached entries.
	pub WeightedSize:usize,
}
