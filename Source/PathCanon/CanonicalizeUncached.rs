use std::path::{Path, PathBuf};

/// Canonicalises `Path` without caching the result.
///
/// Useful for one-shot calls where the resolved path is immediately
/// discarded; avoids polluting the cache with paths that will not
/// repeat.
///
/// # Parameters
///
/// * `Path` — the lexical path to resolve.
///
/// # Returns
///
/// The canonical [`PathBuf`] on success.
///
/// # Errors
///
/// Propagates I/O errors from `dunce::canonicalize`.
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use land_cache::PathCanon::CanonicalizeUncached;
///
/// let resolved = CanonicalizeUncached::Fn(Path::new("/tmp/some-file")).unwrap();
/// ```
pub fn Fn(Path:&Path) -> std::io::Result<PathBuf> { dunce::canonicalize(Path) }
