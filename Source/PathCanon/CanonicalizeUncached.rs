//! Canonicalise without caching. For one-shot calls where the result
//! is immediately discarded; avoids polluting the cache with paths
//! that will not repeat.

use std::path::{Path, PathBuf};

/// Canonicalise `Path` without caching the result.
///
/// Useful for one-shot calls where the resolved path is immediately
/// discarded; avoids polluting the cache with paths that will not
/// repeat.
///
/// # Errors
///
/// Propagates I/O errors from `dunce::canonicalize`.
pub fn Fn(Path:&Path) -> std::io::Result<PathBuf> { dunce::canonicalize(Path) }
