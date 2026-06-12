//! Process-global asset cache backing store. Lazily initialised on
//! first request.

use std::{path::PathBuf, sync::Arc};

use dashmap::DashMap;

use crate::AssetMemoryMap::Entry;

/// Return or lazily initialise the process-global asset cache map.
///
/// The map is a [`DashMap`] keyed by [`PathBuf`] with
/// [`Arc`]<[`Entry::Struct`]> values, created once on first access via
/// [`OnceLock`].
pub fn Fn() -> &'static DashMap<PathBuf, Arc<Entry::Struct>> {
	use std::sync::OnceLock;

	static MAP:OnceLock<DashMap<PathBuf, Arc<Entry::Struct>>> = OnceLock::new();

	MAP.get_or_init(DashMap::new)
}
