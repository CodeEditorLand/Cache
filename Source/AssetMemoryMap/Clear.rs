use crate::AssetMemoryMap::Map;

/// Clears every entry from the asset cache.
///
/// Called on shutdown or on an explicit flush signal from the
/// embedder. After this call the cache is empty; subsequent requests
/// will reload and memory-map files on first access.
///
/// # Examples
///
/// ```rust,no_run
/// use land_cache::AssetMemoryMap::Clear;
///
/// Clear::Fn();
/// ```
pub fn Fn() { Map::Fn().clear(); }
