//! Clear the entire asset cache. Called on shutdown or on an explicit
//! flush signal.

use crate::AssetMemoryMap::Map;

pub fn Fn() { Map::Fn().clear(); }
