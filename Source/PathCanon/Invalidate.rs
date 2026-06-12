use std::path::Path;

use crate::PathCanon::Cache::CACHE;

/// Force-evicts a single path from the canonical-path cache.
///
/// Called from `notify` watchers when a path rename is observed inside
/// the workspace, or by the dev-mode hot-reload signal.
///
/// # Parameters
///
/// * `Path` — the lexical path whose cached canonicalisation should be
///   removed.
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use land_cache::PathCanon::Invalidate;
///
/// Invalidate::Fn(Path::new("/tmp/old-path"));
/// ```
pub fn Fn(Path:&Path) { CACHE.invalidate(Path); }
