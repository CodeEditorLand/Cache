# Changelog - Cache

Cache is our process-wide caching crate - the pair of in-memory caches
`Mountain` reaches for so it doesn't re-read the same workbench asset off disk
or re-canonicalize the same file path twice in one session. This file records
what we built in our voice, version by version. Format adapted from
[Keep a Changelog](https://keepachangelog.com/).

## [v0.1] - Documentation & Architecture Pass (June 2026)

Cache's Rust surface had been stable for weeks; this window was about making the
crate legible - to `cargo doc`, to the README, and to anyone reading the source
for the first time - without touching the caching behavior itself.

### Added

- **README expanded with full architecture documentation and badge labels**
  (`944d836`, 2026-06-23) - Core Architecture Principles table, System
  Architecture diagram, Key Components, and Project Structure sections written
  out in full.
- **README with GitHub badges and architecture doc link** (`cb89ee9`,
  2026-06-12) - first pass at the README beyond the bare doc comment dump from
  `c98b6b7`.
- **Structured rustdoc examples and field docs** added to the public API
  (`f152cfc`, 2026-06-12).
- **`AssetMemoryMap` module documented** with rustdoc comments (`1192b13`,
  2026-06-12).
- **`PathCanon` submodule documented** with rustdoc comments (`0c81e94`,
  2026-06-12).

### Changed

- **ETag derivation**: `AssetMemoryMap`'s `Entry::Struct` now derives its `ETag`
  from the backing file's mtime and size at load time (`327d73f`, 2026-06-12)
  instead of leaving the field unset.
- **Doc comment voice standardized** across modules (`cb41f34`, 2026-06-12).
- **Whitespace in doc examples and module docs normalized** (`59836f0`,
  2026-06-12).
- **Debug `log` macro arguments reformatted** for readability (`c42b25b`,
  2026-05-28).
- **Documentation comments reformatted** for improved readability (`34dad93`,
  2026-05-28).

## [v0.0] - Initial Scaffold (May 2026)

First commit of the crate (`16e7a93`, 2026-05-28): the `AssetMemoryMap` and
`PathCanon` modules landed together as a single-purpose Rust crate consumed by
`Mountain`, along with the first README (`c98b6b7`, 2026-05-28).

---

For the full commit-level history, see the
[commit log](https://github.com/CodeEditorLand/Cache/commits/Current).
