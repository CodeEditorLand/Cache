use memmap2::Mmap;

/// Single memory-mapped asset cache entry.
///
/// Holds the file-backed [`Mmap`](memmap2::Mmap) plus metadata
/// (MIME type, content length, optional brotli sibling, ETag) computed
/// once at load time. The caller must keep the [`Arc`]<[`Struct`]>
/// alive for the lifetime of any response body that borrows the mapping
/// slice.
///
/// # Fields
///
/// * `Mapping` — the memory-mapped file contents.
/// * `Mime` — cached MIME type derived from the file extension.
/// * `Length` — file size at memory-map time.
/// * `Brotli` — optional pre-compressed brotli sibling mapping.
/// * `ETag` — weak validator from file mtime + size.
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

	/// Weak validator derived from file mtime + size at MemoryMap time
	/// (`W/"<mtime-ms-hex>-<size-hex>"`). Usable directly as an `ETag`
	/// response header value for `If-None-Match` revalidation.
	pub ETag:String,
}

impl Struct {
	/// Borrows the entire mapping as a byte slice. The caller must keep the
	/// [`Arc`]<[`Struct`]> alive for the lifetime of any response body that
	/// captures the slice.
	pub fn AsSlice(&self) -> &[u8] { &self.Mapping[..] }

	/// Borrows the brotli-precompressed sibling mapping, if present.
	pub fn AsBrotliSlice(&self) -> Option<&[u8]> { self.Brotli.as_ref().map(|M| &M[..]) }

	/// Returns the byte length of the brotli-precompressed sibling, if any.
	///
	/// Useful for setting the `Content-Length` header when serving the
	/// precompressed payload.
	pub fn BrotliLength(&self) -> Option<usize> { self.Brotli.as_ref().map(|M| M.len()) }
}
