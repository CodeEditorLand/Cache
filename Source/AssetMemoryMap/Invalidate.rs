//! Drops a single cached entry. Useful for hot-reload during development
//! when the bundler rewrites a chunk.

use std::{path::Path, sync::Arc};

use crate::AssetMemoryMap::{Entry, Map};

/// Drops a single cached entry, returning it if it existed.
///
/// Useful for hot-reload during development when the bundler rewrites
/// a chunk; the caller can inspect or discard the old entry.
pub fn Fn(Path:&Path) -> Option<Arc<Entry::Struct>> { Map::Fn().remove(Path).map(|(_, V)| V) }
