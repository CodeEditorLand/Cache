//! Process-global canonical-path cache backing store.

use std::{path::PathBuf, time::Duration};

use moka::sync::Cache;
use once_cell::sync::Lazy;

/// Process-global canonical-path cache backed by `moka::sync::Cache`.
///
/// Keyed by lexical [`PathBuf`]; values are the resolved canonical
/// [`PathBuf`]. Configured with 8 192 max capacity and a 60-second
/// time-to-idle that resets on each access.
pub static CACHE:Lazy<Cache<PathBuf, PathBuf>> = Lazy::new(|| {
	Cache::builder()
		.max_capacity(8192)
		.time_to_idle(Duration::from_secs(60))
		.build()
});
