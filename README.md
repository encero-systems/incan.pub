# incan.pub

The registry for Oven-built projects, as specified by [RFC 125](https://github.com/encero-systems/incan/blob/main/workspaces/docs-site/docs/RFCs/125_incan_pub_loaf_registry_and_baked_asset_distribution.md). This repository is its interim home. Today it holds one kind of content: **adoption manifests** for crates.io packages.

## What an adoption manifest is

Oven never executes a package's `build.rs`. Under [RFC 119](https://github.com/encero-systems/incan/blob/main/workspaces/docs-site/docs/RFCs/119_oven_native_rust_build_facets_and_cargo_interoperation.md) a build script is inert source inventory; what it would have discovered is declared instead, as `cfg` answers, committed `out` inputs, `link` work and `tool` work. A crates.io package graduates to Oven-native when it has a `loaf.toml` whose declared facts cover its build inputs. Upstream packages do not ship that manifest, so this repository holds it for them: one `loaf.toml` per package version, describing exactly one crates.io source by checksum.

Each manifest is a set of **bound fact records**. A record states the facts for one exact selection — toolchain, target, profile and enabled features — because a probe's answer is a constant only under that selection. `libm` enables `optimizations_enabled` above opt-level 1, so its release and debug records differ; `proc-macro2` answers by compiler version, so every record names the compiler. A consumer applies a record only when its own selection matches the record's binding and refuses otherwise. It never interpolates between records and never falls back to running the script.

## What this repository is not

- Not a mirror of crates.io. No package source is stored here; a manifest binds to the crates.io source by checksum, and the source is fetched from the registry as before.
- Not executable. Nothing here runs during resolution or baking, as RFC 125 requires. Committed `out` files are inputs the compiler reads, never programs.
- Not a directive interpreter. A record is the answer, not the question: it does not restate `rerun-if-*`, `rustc-check-cfg`, `links`, `DEP_*` or script-emitted warnings, none of which are Oven authority.

## Layout

```
index/...                              static sparse index, one JSON line per package version (crates.io path scheme)
crates-io/<name>/<version>/loaf.toml   the adoption manifest for one crates.io package version
crates-io/<name>/<version>/out/...     committed generated inputs a record names
docs/adoption-manifest.md              the manifest format
scripts/registry.py                    builds the index from the manifests and validates the repository
```

The index is a projection of the manifests and is never edited by hand: `scripts/registry.py build` regenerates it, and `scripts/registry.py check` (run in CI on every change) refuses a repository whose manifests, committed generated inputs, or index disagree. A client reads the index line for a package, verifies the crates.io checksum it names, fetches the manifest, and selects the record whose binding equals its own selection.

## How facts get here

A fact is harvested, not guessed. Oven's Cargo-compatibility publisher already observes each unit's build-script directives and generated outputs under one exact selection; a record is proposed from that observation and proven by literal artifact equivalence against the Cargo-built unit. A harvest is only a fact of the toolchain when nothing ambient influenced it: a probe answered under `RUSTC_BOOTSTRAP`, for example, is not a stable compiler's answer and must not be recorded as one.

## Status

Interim and versioned by commit. The Incan toolchain pins the commit it consumes. When RFC 125's signed index and events exist, these manifests become published, attested registry content; their meaning does not change.
