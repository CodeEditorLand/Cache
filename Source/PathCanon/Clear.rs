//! Clears the entire path-canon cache. Diagnostic / shutdown use.

use crate::PathCanon::Cache::CACHE;

/// Clears every entry from the path-canon cache.
///
/// Typically called on shutdown or in response to a diagnostic signal.
pub fn Fn() { CACHE.invalidate_all(); }
