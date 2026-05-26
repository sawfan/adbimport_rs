//! WASM/browser-oriented loading pattern for a `.rkyv` genealogy archive.
//!
//! This example is intentionally "no-std-web" (it does not depend on `web-sys`
//! or `gloo`). It demonstrates the *ownership* pattern you should use:
//! - Download / fetch the `.rkyv` file into a `Vec<u8>` (how you do that is
//!   app-specific).
//! - Construct `GenealogyStore::from_bytes(bytes)`.
//! - Keep the `GenealogyStore` alive for as long as you need to access archived
//!   references.
//!
//! In WASM you typically fetch bytes asynchronously; once you have the `Vec<u8>`
//! you can feed it into the store.

use adbimport::genealogy::GenealogyStore;

/// Demonstrates the intended in-browser usage where the store owns the archive bytes.
///
/// In a real WASM app, `bytes` would come from a fetch request.
pub fn load_store_from_bytes(bytes: Vec<u8>) -> Result<GenealogyStore, Box<dyn std::error::Error>> {
    Ok(GenealogyStore::from_bytes(bytes)?)
}

/// A small query helper that returns owned data (safe to cross async/UI boundaries).
///
/// Note how we do *not* return `&Archived<_>`; the returned `Vec<String>` is
/// independent of the archive lifetime.
pub fn top_names(
    store: &GenealogyStore,
    query: &str,
    limit: usize,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let hits = store.search_people_by_name(query, limit)?;

    let mut out = Vec::with_capacity(hits.len());
    for p in hits {
        let name = p
            .names
            .first()
            .map(|n| n.display.as_str())
            .unwrap_or("<no name>");
        out.push(name.to_owned());
    }

    Ok(out)
}

fn main() {
    // This is a compile-check example only.
    //
    // To use it for real, load bytes from a `.rkyv` file (or fetch in WASM) and
    // pass them into `load_store_from_bytes`.
}
