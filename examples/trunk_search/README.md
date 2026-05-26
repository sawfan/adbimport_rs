# Trunk browser search example

This is a minimal Trunk + WASM demo that loads a serialized `.rkyv` genealogy archive in the browser and performs name searching.

## Prepare the archive asset

From the repo root, generate a `.rkyv` archive:

```bash
cargo run -- --format rkyv assets/c_sample.xml out.rkyv
```

Copy it into the Trunk app's public dir:

```bash
cp out.rkyv examples/trunk_search/public/sample.rkyv
```

## Run with Trunk

From `examples/trunk_search`:

```bash
trunk serve --open
```

Trunk copies `public/` into the dist output (via `rel="copy-dir"` in `index.html`), so the archive is available at:
- `http://127.0.0.1:8080/public/sample.rkyv`

A quick sanity check is that this URL should **download a large binary file** (not return `index.html`).

Then search for a name (the query uses tokenization + AND semantics).

