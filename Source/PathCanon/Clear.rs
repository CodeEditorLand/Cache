use crate::PathCanon::Cache::CACHE;

/// Clears every entry from the path-canon cache.
///
/// Typically called on shutdown or in response to a diagnostic signal.
/// After this call the cache is empty; subsequent lookups will miss and
/// re-populate entries.
///
/// # Examples
///
/// ```rust,no_run
/// use land_cache::PathCanon::Clear;
///
/// Clear::Fn();
/// ```
pub fn Fn() { CACHE.invalidate_all(); }
