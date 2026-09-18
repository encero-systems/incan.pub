# incan.pub

The registry for Oven-built projects, as specified by [RFC 125](https://github.com/encero-systems/incan/blob/main/workspaces/docs-site/docs/RFCs/125_incan_pub_loaf_registry_and_baked_asset_distribution.md). This repository is its interim home, in the repository-based shape [docs/v0.md](docs/v0.md) defines: this branch holds the tool and the docs; the [`index`](https://github.com/encero-systems/incan.pub/tree/index) branch holds the data. Today the registry holds one kind of publication: **external-source records** for crates.io packages, and the event log they are folded from. [docs/model.md](docs/model.md) states what Incan does to publish and what the registry does to manage.

## What an external-source record is

Oven never executes a package's `build.rs`. Under [RFC 119](https://github.com/encero-systems/incan/blob/main/workspaces/docs-site/docs/RFCs/119_oven_native_rust_build_facets_and_cargo_interoperation.md) a build script is inert source inventory; what it would have discovered is declared instead, as `cfg` answers, committed `out` inputs, `link` work and `tool` work. A crates.io package graduates to Oven-native when it has a `loaf.toml` whose declared facts cover its build inputs. Upstream packages do not ship that manifest, so this registry publishes it for them: an external-source record per package version, describing exactly one crates.io source by checksum. RFC 125 names this object as one of the two publications a baked asset may derive from.

Each record is a set of **bound fact records**. A record states the facts for one exact selection — toolchain, target, profile and enabled features — because a probe's answer is a constant only under that selection. `libm` enables `optimizations_enabled` above opt-level 1, so its release and debug records differ; `proc-macro2` answers by compiler version, so every record names the compiler. A consumer applies a record only when its own selection matches the record's binding and refuses otherwise. It never interpolates between records and never falls back to running the script.

## What this repository is not

- Not a mirror of crates.io. No package source is stored here; a record binds to the crates.io source by checksum, and the source is fetched from the registry as before.
- Not executable. Nothing here runs during resolution or baking, as RFC 125 requires. Committed `out` files are inputs the compiler reads, never programs.
- Not a directive interpreter. A record is the answer, not the question: it does not restate `rerun-if-*`, `rustc-check-cfg`, `links`, `DEP_*` or script-emitted warnings, none of which are Oven authority.

## Layout

```
main
  incan.toml, src/, tests/                 incan-pub: admission (`check`), projection (`build`), the publishing verbs
  build.sh                                 test and build the tool with the installed Incan toolchain → target/incan-pub
  scripts/snapshot-crates-io-index.sh      fetch the crates.io index lines the log names, for `check --crates-io-index`
  docs/v0.md                               the v0 design: repository-based, two branches, three programs
  docs/model.md                            the loop, from both ends
  docs/external-source-record.md           the record, its events, and the rendered manifest
  docs/harvest-proposal.md                 what `oven harvest` emits and `add-fact` admits
  docs/explanation.html                    the explanation page (self-contained HTML, incapunk styling)

index
  events/NNNNNN-<kind>-<subject>.json      the append-only event log; the only thing that is authored
  crates-io/<name>/<version>/loaf.toml     projection: the external-source record as a consumer reads it
  crates-io/<name>/<version>/out/...       committed generated inputs a record names
  index/...                                projection: static sparse index, one JSON line per package version
```

Nothing under `crates-io/**/loaf.toml` or `index/` is edited by hand: `incan-pub build` regenerates both from the events, and `incan-pub check` — run in CI on every push to either branch — refuses a checkout whose events, committed generated inputs or projections disagree, and verifies every record's checksum against the crates.io index. A client reads the index line for a package, verifies the checksum it names, reads the rendered record, and selects the fact whose binding equals its own selection; the line also says whether that binding is merely `harvested` or `attested`.

In this transport a commit is the signed event and `HEAD` of `index` is the checkpoint the toolchain pins. The signed HTTPS form of RFC 125 changes how these files arrive, not what they say.

## Using the tool

```sh
./scripts/prewarm-toolchain.sh                # once per fresh 0.5.1 install (encero-systems/incan#1667)
./build.sh                                   # incan oven bake, incan test, incan build → target/incan-pub
git worktree add ../incan.pub-index index    # a checkout of the data branch
cd ../incan.pub-index
../incan.pub/target/incan-pub check          # admit the log, compare the projections
../incan.pub/scripts/snapshot-crates-io-index.sh ../incan.pub/target/incan-pub .crates-io-snapshot
../incan.pub/target/incan-pub check --crates-io-index .crates-io-snapshot
```

`publish NAME VERSION CHECKSUM [--notes FILE]` creates a record, `add-fact PROPOSAL.json` admits a harvested fact, `attest …` records an equivalence-attested asset; each appends one event and rebuilds the projections. `--root DIR` names the checkout when it is not the current directory. The tool is written in Incan and built by the released toolchain; see the last section of [docs/v0.md](docs/v0.md) for why that is a rule.

## How facts get here

A fact is harvested, not guessed. Oven's Cargo-compatibility publisher observes each unit's build-script directives and generated outputs under one exact selection; `oven harvest` emits that observation as a [proposal](docs/harvest-proposal.md), and `incan-pub add-fact` admits it as a `fact` event. A binding becomes `attested` when an Oven-baked unit is proven equivalent to the Cargo-built one. A harvest is only a fact of the toolchain when nothing ambient influenced it: a probe answered under `RUSTC_BOOTSTRAP`, for example, is not a stable compiler's answer and must not be recorded as one.

## Status

Interim and versioned by commit. The Incan toolchain pins the `index` commit it consumes. When RFC 125's signed index and events exist, these records become published, attested registry content; their meaning does not change.
