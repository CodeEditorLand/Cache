//! Single MemoryMap-backed asset cache entry. Holds the file-backed
//! mapping plus metadata computed once at load time.

use memmap2::Mmap;

/// Single memory-mapped asset cache entry.
///
/// Holds the file-backed [`Mmap`](memmap2::Mmap) plus metadata computed
/// once at load time. The caller must keep the `Arc<Struct>` alive for
/// the lifetime of any response body that borrows the mapping slice.
pub struct Struct {
	/// The MemoryMap mapping itself. Keep alive as long as any webview
	/// body references it.
	pub Mapping:Mmap,

	/// Cached MIME from the file extension. Avoids the match arm on
	/// the hot path.
	pub Mime:&'static str,

	/// File size at MemoryMap time. Used for `Content-Length`.
	pub Length:usize,

	/// Optional pre-brotli-compressed sibling (path with `.br`
	/// suffix). `None` if no sibling existed at load time.
	pub Brotli:Option<Mmap>,
}

impl Struct {
	/// Borrow the entire mapping as a slice. Caller keeps
	/// `Arc<Struct>` alive for the lifetime of any response body that
	/// captures the slice.
	pub fn AsSlice(&self) -> &[u8] { &self.Mapping[..] }

	/// Borrow the brotli-precompressed sibling if present.
	pub fn AsBrotliSlice(&self) -> Option<&[u8]> { self.Brotli.as_ref().map(|M| &M[..]) }

	/// Length of the brotli sibling. Useful for `Content-Length` when
	/// serving the precompressed payload.
	pub fn BrotliLength(&self) -> Option<usize> { self.Brotli.as_ref().map(|M| M.len()) }
}
