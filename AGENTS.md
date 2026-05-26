# AGENTS

## Log

- Updated the `adbimport` CLI to support multiple output formats via `--format`.
  - `--format gedcom7` (default): XML -> GEDCOM 7
  - `--format rkyv`: XML -> structured genealogy index -> rkyv archive bytes
- Fixed rkyv derive/attribute usage to match rkyv 0.8 (`#[rkyv(...)]`) and ensured archived ID types derive ordering traits needed for archived `BTreeMap` keys.
- Addressed clippy warnings so `cargo clippy -- -D warnings` passes.
- Added a consumer-facing `GenealogyStore` API that owns archive bytes and provides safe access/query helpers (from bytes or from file).
- Added examples:
  - `examples/query_rkyv.rs`: native CLI-style querying of `.rkyv` (person lookup, name search, events by year)
  - `examples/wasm_load_rkyv.rs`: browser/WASM-oriented loading pattern that keeps bytes alive and returns owned query results
- Added a Trunk browser/WASM example app at `examples/trunk_search` that fetches `public/sample.rkyv` and demonstrates interactive name searching.
- Made the genealogy archive loader/store WASM-friendly by gating filesystem APIs behind `cfg(not(target_arch = "wasm32"))`.
- Fixed WASM loading issue by having `GenealogyStore` copy input bytes into an `AlignedVec` before accessing the archive.

- Enhanced the rkyv genealogy model + parsers to preserve birth time/timezone and astro positions (sun/moon/asc) on Birth events.
- Enhanced the structured parser + place model to capture latitude/longitude from Astrodatabank `<place slati="..." slong="...">` attributes (best-effort parsing of formats like `52n15`, `21e0`).
- Improved example output formatting:
  - `examples/query_rkyv.rs`: prints people/event lookups and name search results in aligned ASCII tables including birth date/time and sun/moon/asc when present.
  - `examples/trunk_search`: renders name search results as an HTML table with the same columns.


## Commands

- Fixed the Trunk WASM example build by adding the missing `rkyv` dependency and updating archived ID conversions.
- Regenerated `examples/trunk_search/public/sample.rkyv` to match the updated archive schema.

- Run tests: `cargo test`
- Generate GEDCOM 7:
  - `cargo run -- assets/c_sample.xml out.ged`
- Generate rkyv archive:
  - `cargo run -- --format rkyv assets/c_sample.xml out.rkyv`

