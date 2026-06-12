/// Occupancy snapshot returned by [`super::Stats::Fn`].
///
/// Records the entry count, brotli-sibling availability, and total
/// bytes across all cached assets at the moment the snapshot was taken.
///
/// # Examples
///
/// ```rust,no_run
/// use land_cache::AssetMemoryMap::CacheStats;
///
/// let stats = CacheStats::Struct {
/// 	Entries:128,
/// 	BrotliEntries:96,
/// 	Bytes:80_000_000,
/// 	BrotliBytes:32_000_000,
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Struct {
	/// Number of entries currently in the cache.
	pub Entries:usize,

	/// Number of entries that have a brotli-precompressed sibling.
	pub BrotliEntries:usize,

	/// Total uncompressed bytes across all entries.
	pub Bytes:usize,

	/// Total precompressed brotli bytes across all brotli entries.
	pub BrotliBytes:usize,
}
