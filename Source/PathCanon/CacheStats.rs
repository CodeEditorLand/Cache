//! Diagnostic snapshot of the canonical-path cache.

/// Diagnostic snapshot of the canonical-path cache.
#[derive(Debug, Clone, Copy)]
pub struct Struct {
	/// Number of entries currently held in the cache.
	pub Entries:usize,

	/// Total weighted size (byte-estimate) of all cached entries.
	pub WeightedSize:usize,
}
