//! Clear the entire path-canon cache. Diagnostic / shutdown use.

use crate::PathCanon::Cache::CACHE;

pub fn Fn() { CACHE.invalidate_all(); }
