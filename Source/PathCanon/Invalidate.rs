//! Force-evicts an entry. Called from `notify` watchers when a path
//! rename is observed inside the workspace, or by the dev-mode
//! hot-reload signal.

use std::path::Path;

use crate::PathCanon::Cache::CACHE;

/// Force-evicts a single path from the canonical-path cache.
///
/// Called from `notify` watchers when a path rename is observed inside
/// the workspace, or by the dev-mode hot-reload signal.
pub fn Fn(Path:&Path) { CACHE.invalidate(Path); }
