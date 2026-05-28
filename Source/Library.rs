#![allow(
	non_snake_case,
	non_camel_case_types,
	non_upper_case_globals,
	dead_code,
	unused_imports,
	unused_variables,
	unused_assignments
)]

//! # Cache 📦 - process-wide caching primitives for Land 🏞️
//!
//! Two independent caches that the Tauri host (Mountain) and any other
//! Land embedder share:
//!
//! - [`AssetMemoryMap`] - file-backed mmap cache for bundled static
//!   assets. The bundled workbench under
//!   `Element/Sky/Target/Static/Application/` is ~80 MB; per-request
//!   `fs::read` pays a syscall + alloc + memcpy on every fetch, whereas
//!   `memmap2::Mmap` hands the webview a borrowed slice of file-backed
//!   pages the OS can evict under pressure. Optional `<file>.br`
//!   siblings are picked up transparently for `Content-Encoding: br`.
//! - [`PathCanon`] - process-wide canonical-path cache. Collapses
//!   repeated `dunce::canonicalize` calls used by fs-scope security
//!   gates. `time_to_idle = 60s` bounds staleness against external
//!   renames while hot paths stay cached indefinitely.
//!
//! Both caches are additive performance helpers; consumers continue to
//! function with any one of them disabled.

pub mod AssetMemoryMap;

pub mod PathCanon;
