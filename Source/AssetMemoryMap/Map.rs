use std::{path::PathBuf, sync::Arc};

use dashmap::DashMap;

use crate::AssetMemoryMap::Entry;

/// Returns or lazily initialises the process-global asset cache map.
///
/// The map is a [`DashMap`] keyed by [`PathBuf`] with
/// [`Arc`]<[`Entry::Struct`]> values, created once on first access via
/// [`OnceLock`]. This ensures the map is initialised exactly once,
/// even under concurrent access.
///
/// # Returns
///
/// A reference to the static [`DashMap`] instance.
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::PathBuf;
///
/// use land_cache::AssetMemoryMap::Map;
///
/// let map = Map::Fn();
/// map.insert(PathBuf::from("key") /* Arc<Entry::Struct> */);
/// ```
pub fn Fn() -> &'static DashMap<PathBuf, Arc<Entry::Struct>> {
	use std::sync::OnceLock;

	static MAP:OnceLock<DashMap<PathBuf, Arc<Entry::Struct>>> = OnceLock::new();

	MAP.get_or_init(DashMap::new)
}
