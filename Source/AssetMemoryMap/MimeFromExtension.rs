use std::path::Path;

/// Maps a file extension to its IANA media type string.
///
/// Mirrors the inline helper Mountain uses in
/// `Binary/Build/Scheme.rs` so the cache layer is self-contained.
/// Unknown extensions fall back to `application/octet-stream`.
///
/// # Parameters
///
/// * `Path` — the file path whose extension determines the MIME type.
///
/// # Returns
///
/// A `&'static str` with the IANA media type (and charset where
/// applicable), or `application/octet-stream` for unknown extensions.
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use land_cache::AssetMemoryMap::MimeFromExtension;
///
/// let mime = MimeFromExtension::Fn(Path::new("index.html"));
/// assert_eq!(mime, "text/html; charset=utf-8");
/// ```
pub fn Fn(Path:&Path) -> &'static str {
	match Path.extension().and_then(|S| S.to_str()).unwrap_or("") {
		"js" | "mjs" | "cjs" => "application/javascript; charset=utf-8",

		"css" => "text/css; charset=utf-8",

		"html" | "htm" => "text/html; charset=utf-8",

		"json" | "map" => "application/json; charset=utf-8",

		"svg" => "image/svg+xml",

		"png" => "image/png",

		"jpg" | "jpeg" => "image/jpeg",

		"gif" => "image/gif",

		"webp" => "image/webp",

		"woff" => "font/woff",

		"woff2" => "font/woff2",

		"ttf" => "font/ttf",

		"otf" => "font/otf",

		"wasm" => "application/wasm",

		"ico" => "image/x-icon",

		"txt" => "text/plain; charset=utf-8",

		"md" => "text/markdown; charset=utf-8",

		_ => "application/octet-stream",
	}
}
