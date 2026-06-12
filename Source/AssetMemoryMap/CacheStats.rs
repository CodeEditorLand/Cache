//! Snapshot of asset-cache occupancy, returned by [`super::Stats::Fn`].

/// Occupancy snapshot returned by [`super::Stats::Fn`]. Records entry
/// count, brotli-sibling availability, and total bytes.
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
