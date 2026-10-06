# incan.pub

The registry for Oven-built projects, as specified by [RFC 125](https://github.com/encero-systems/incan/blob/main/workspaces/docs-site/docs/RFCs/125_incan_pub_loaf_registry_and_baked_asset_distribution.md). This repository is its interim home, in the repository-based shape [docs/v0.md](docs/v0.md) defines: this branch holds the tool and the docs; the [`index`](https://github.com/encero-systems/incan.pub/tree/index) branch holds the data. Today the registry holds crates.io packages **adopted as Loaves**, the **bound build facts** recorded for them, and the event log both are folded from. [docs/model.md](docs/model.md) states what Incan does to publish and what the registry does to manage.

## What an adopted Loaf is

Incan does nothing with Cargo: every third-party package reaches Oven as a Loaf. Adoption turns one crates.io package version into a Loaf once. It reads the version's crates.io index line and its `.crate` archive, verified against the crates.io checksum, and from the archive's manifest only the edition and the library target. It writes a `loaf.toml` in [RFC 119](https://github.com/encero-systems/incan/blob/main/workspaces/docs-site/docs/RFCs/119_oven_native_rust_build_facets_and_cargo_interoperation.md)'s Rust facet grammar: the `[rust]` table, the features with implicit optional features made explicit, and the normal dependencies as `crates-io/<name>` Loaf dependencies; development and build dependencies and `links` drop. The source Loaf archive is that manifest plus the package's files unchanged, in the deterministic tar form RFC 125 fixes, and its digest is the Loaf's identity. Admission refuses an adoption while a dependency active under its default features, on any target, is not adopted yet, so every `index` commit resolves any default build on its own; a dependency only a further feature enables is adopted when someone enables it.

## What a bound build fact is

Oven never executes a package's `build.rs`. What a build script would have discovered is declared instead, as `cfg` answers, committed `out` inputs, `link` work and `tool` work. A record states those facts for one exact selection — toolchain, target, profile and complete feature set — because a probe's answer is a constant only under that selection. `libm` enables `optimizations_enabled` above opt-level 1, so its release and debug facts differ; `proc-macro2` answers by compiler version, so every binding names the compiler. A consumer applies a fact only when its own selection matches the binding and refuses otherwise. It never interpolates between bindings and never falls back to running the script.

## What this repository is not

- Not a mirror of crates.io. Only the versions Incan users need are adopted, each with provenance back to its crates.io checksum. Source Loaf archives live in a blob store by digest, not in git.
- Not executable. Nothing here runs during resolution or baking. Committed `out` files are inputs the compiler reads, never programs.
- Not a directive interpreter. A fact is the answer, not the question: it does not restate `rerun-if-*`, `rustc-check-cfg`, `links`, `DEP_*` or script-emitted warnings, none of which are Oven authority.

## Layout

```
main
  loaf.toml, src/, tests/                  incan-pub: admission (`check`), projection (`build`), adoption, the publishing verbs
  build.sh                                 test and build the tool with an Incan toolchain → target/incan-pub
  scripts/snapshot-crates-io-index.sh      fetch the crates.io index lines the log names, for `check --crates-io-index`
  docs/                                    the v0 design, the loop, the record, the harvest proposal

index
  events/NNNNNN-<kind>-<subject>.json      the append-only event log; the only thing that is authored
  index/crates-io/<name>                   projection: one JSON line per version, enough to resolve the whole graph
  crates-io/<name>/<version>/loaf.toml     projection: the adopted manifest followed by its bound facts
  crates-io/<name>/<version>/out/...       committed generated inputs a fact names
  graphs/<hex>.json                        the resolutions facts were selected from: roots, target, host

blob store (outside git)
  sha256/<hex>.tar                         source Loaf archives by digest
```

Nothing under `index/` or `crates-io/**/loaf.toml` is edited by hand: `incan-pub build` regenerates both from the events, and `incan-pub check` — run in CI on every push to either branch — refuses a checkout whose events, committed generated inputs or projections disagree. With `--crates-io-index` it verifies every crates.io checksum, and with `--blobs` every source Loaf archive. An index line carries the version's `rust` table, `deps`, `features` and `adopted` provenance, its archive digest as `cksum`, and the status of each binding: `harvested` or `attested`.

In this transport a commit is the signed event and `HEAD` of `index` is the checkpoint the toolchain pins. The signed HTTPS form of RFC 125 changes how these files arrive, not what they say.

## Using the tool

```sh
INCAN=/path/to/incan ./build.sh               # lock, bake, test, build → target/incan-pub
git worktree add ../incan.pub-index index     # a checkout of the data branch
cd ../incan.pub-index
../incan.pub/target/incan-pub check           # admit the log, compare the projections
../incan.pub/target/incan-pub adopt --records --cache <cache> --blobs <blob store>
```

`adopt [NAME VERSION]... [--records]` adopts versions and their dependency closure, and `readopt NAME VERSION ...` rewrites an adoption that is wrong because of a bug. `publish NAME VERSION CHECKSUM` creates a v0 record, `add-fact PROPOSAL.json [--publish]` admits a harvested fact, `admit-drop DIR` admits a whole harvest drop after cross-checking it, `attest …` records an equivalence-attested asset, `advise NAME VERSION --text TEXT` records something a consumer should know. Each appends events and rebuilds the projections; `--root DIR` names the checkout when it is not the current directory. The tool is written in Incan and built on the Incan dev line; [AGENTS.md](AGENTS.md) says how.

## How facts get here

A fact is harvested, not guessed. A harvest observes each unit's build-script directives and generated outputs under one exact selection and emits that observation as a [proposal](docs/harvest-proposal.md), which `incan-pub add-fact` admits as a `fact` event. A binding becomes `attested` when an Oven-baked unit is proven equivalent. A harvest is only a fact of the toolchain when nothing ambient influenced it: a probe answered under `RUSTC_BOOTSTRAP`, for example, is not a stable compiler's answer and must not be recorded as one.

## Status

Interim and versioned by commit. The Incan toolchain pins the `index` commit it consumes. When RFC 125's signed index and events exist, these records become published, attested registry content; their meaning does not change.
