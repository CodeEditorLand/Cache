//! Clears the entire asset cache. Called on shutdown or on an explicit
//! flush signal.

use crate::AssetMemoryMap::Map;

/// Clears every entry from the asset cache.
///
/// Called on shutdown or on an explicit flush signal from the
/// embedder.
pub fn Fn() { Map::Fn().clear(); }
