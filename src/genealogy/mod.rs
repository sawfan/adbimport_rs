//! Astrodatabank → Kleio adapter.
//!
//! The canonical genealogy primitives live in the `kleio` crate.
//!
//! This module keeps:
//! - ADB-specific parsing (`parse_astrodatabank`) that converts ADB XML into `kleio` types.
//! - ADB serialization helpers (building a Kleio archive for bundling into apps).

pub mod parse_astrodatabank;

pub use parse_astrodatabank::*;
