//! # PathCanon
//!
//! Process-wide canonical-path cache keyed by lexical input path.
//! Values are the result of `dunce::canonicalize`; hits skip the
//! syscall, misses run it and cache the result.
//!
//! The fs-scope security gates that Mountain layers around every
//! incoming path canonicalise repeatedly during boot: 113 extension
//! manifest paths, ~80 chunked workbench JS imports, ~60 git-extension
//! scope checks, every `vscode-file://` request. Collapsing repeats to
//! a hash lookup saves ~150 ms cumulative on the boot path.
//!
//! `time_to_idle = 60s`: resets on each access, so hot paths stay
//! cached indefinitely while one-shot paths evict naturally. Bounds
//! staleness against external `mv` / rename.

pub mod CacheStats;

pub mod Canonicalize;

pub mod CanonicalizeUncached;

pub mod Clear;

pub mod Invalidate;

pub mod SpawnDiagnosticLogger;

pub mod Stats;

pub(crate) mod Cache;
