# Working on incan.pub

This repository is the interim registry of [RFC 125](https://github.com/encero-systems/incan/blob/main/workspaces/docs-site/docs/RFCs/125_incan_pub_loaf_registry_and_baked_asset_distribution.md), in the repository-based v0 shape [docs/v0.md](docs/v0.md) defines. `main` holds the tool and the docs; the orphan [`index`](https://github.com/encero-systems/incan.pub/tree/index) branch holds the data a toolchain reads and pins by commit.

## Rules

1. **The tool is written in Incan.** Not Python, not shell beyond the two CI helpers. A registry whose admission were written in another language would be a claim the language cannot make about itself.
2. **Projections are never edited.** Change the log under `events/`, run `incan-pub build`, commit both. `incan-pub check` refuses a checkout whose events, committed generated inputs or projections disagree, and CI runs it on both branches.
3. **The fact vocabulary is closed** and mirrors the compiler's `RustFactRecord`; [docs/external-source-record.md](docs/external-source-record.md) states what a key entering it must satisfy. Adding a key here without the compiler reading it publishes a fact no consumer can apply.
4. **The tool is built on the Incan dev line.** No release installs it yet. CI pulls the compiler the `toolchain` workflow built at the commit `.github/workflows/check.yml` pins; a developer machine builds that commit. A compiler gap is worked around in place with a comment naming it, and reported to the dev line rather than patched here.

## Building

```sh
INCAN=/path/to/incan ./build.sh   # lock, bake, test, build -> target/incan-pub
```

`INCAN` is a compiler built from the commit CI pins (`cargo build -p incan-cli` in a checkout of encero-systems/incan, debug, with its own Cargo target directory); the `incan` on `PATH` when unset. Any source change needs a fresh `incan oven bake --project .` before `incan check`, `test` or `run` accepts the project again (encero-systems/incan#2081), and `build.sh` does that. `build.sh` takes its own `INCAN_HOME` under `target/`. The default `~/.incan` is shared with every toolchain on the machine, and a store entry written by another one reads here as "Oven store integrity failure: manifest identity does not match its immutable content". Override `INCAN_HOME` only to reuse a warm store deliberately. The same applies to any Cargo target directory: a compiler binary embeds its repository root, so two checkouts building into one target poison each other.

## Admitting a harvest

A harvest drop is a directory of `<name>-<version>-<profile>/proposal.json` plus the `out/` files each proposal names, produced by the release bake ([docs/harvest-proposal.md](docs/harvest-proposal.md) is the contract). Against a checkout of `index`:

```sh
incan-pub admit-drop <drop>                      # cross-checks the drop, then admits every proposal in order
incan-pub check --crates-io-index <snapshot>     # scripts/snapshot-crates-io-index.sh writes the snapshot
git commit && git push
```

Then report the resulting `index` HEAD: that commit is what a toolchain manifest pins and what the next release bake consumes. A proposal admission refuses goes back to whoever harvested it verbatim — the fix belongs on the harvest side, never in the proposal or the log. An identical binding is a no-op; a different one for a binding already on record is a refusal, and that refusal is the point: it is how a record made by hand gets checked against the machine.

Records the harvest declines to propose keep whatever record they have: a script whose emitted environment it cannot prove is unread, or link or tool work whose closure it cannot bind completely. Do not hand-write a fact to fill the gap — an incomplete declaration is worse than none, because a consumer cannot tell that it is incomplete.

`admit-drop` performs the one check that needs the whole drop: a package named in `refusals-<profile>.json` must not also have a proposal for that profile. Admission cannot catch that contradiction one proposal at a time — a fact that omits work the script does says nothing about the work it omits — so the drop is refused whole, and nothing is admitted. `add-fact` remains for a single proposal.

## Adopting crates.io packages

Adoption turns a crates.io package version into a Loaf ([RFC 125](https://github.com/encero-systems/incan/blob/main/workspaces/docs-site/docs/RFCs/125_incan_pub_loaf_registry_and_baked_asset_distribution.md), "Adoption from crates.io"): one `adopt` event per version, whose manifest follows RFC 119's Rust facet grammar and whose source Loaf archive is stored by digest. Admission refuses an `adopt` event while a dependency active under the version's default features, on any target, is not adopted yet, so `adopt` plans the whole closure first and appends in dependency order. The plan is what a consumer of each named version builds with its default features: it follows the features every dependent enables on a dependency, through versions already adopted as well, so an optional dependency that only a dependent's feature switches on is adopted too, and naming an adopted version again adopts whatever its build still lacks. Against a checkout of `index`:

```sh
incan-pub adopt --records --cache <cache> --blobs <blob store>   # or: adopt NAME VERSION ...
incan-pub check --crates-io-index <snapshot> --blobs <blob store>
git commit && git push
```

`--cache` keeps the crates.io index files and `.crate` archives adoption downloads, each checked against its crates.io checksum before it is read; `--blobs` receives every source Loaf archive as `sha256/<hex>.tar`. A requirement no adopted version meets is met by the newest crates.io version satisfying it, preferring one that is not yanked. Report the resulting `index` HEAD as for a harvest.

## Recording build facts

A version with a build script needs a fact for every binding a consumer selects (RFC 125, "Adoption from crates.io", rule 6). `incan-pub record-fact NAME VERSION --features a,b --profile P --domain target|host --graph <roots.json> --rustc <pinned rustc> --cache <cache> --blobs <blob store> --work <dir>` compiles the script with that `rustc` and runs it under `sandbox-exec` with the network closed, writes confined to its work directory, and an environment built only from the binding. Its `rustc-cfg` answers and `OUT_DIR` files become the fact, which is admitted like any harvested proposal, with the domain and the graph in its evidence. A version without a build script is reported as needing no fact.

A script that compiles C or C++ needs `--cc`, `--cxx`, `--ar` and `--sysroot`: `CC`, `CXX` and `AR` then name copies of `incan-pub` that run those exact tools and trace every successful run, and `SDKROOT` names the sysroot. After the script, the traces become `link` records ([src/native.incn](src/native.incn)): each `static=` library one archive, each archived object one plain compile, crate paths declared inputs, compiler paths owner paths, and the compiler identified by an inventory of everything its owner paths reach. Non-archived compiles are probes, kept as evidence. `incan-pub native-convert` prints what a directory of traces converts to. A script with build dependencies needs `--incan` (a toolchain with `incan oven compile-closure`) and `--build-lock` (the resolution of its build-dependency roots): Oven compiles them for the host from their adopted archives and the script links against what it returns.

An include tree lists only the files the compiles read beneath it, taken from each compile's dependency file, and every file a compile reads must be a declared input or lie in the compiler's owner closure; a file it reads without its arguments naming it is a declared source listed in that object's `reads` (RFC 119). `out` is the `OUT_DIR` files the crate's own sources include. `--rewrite` replaces a binding's fact in place when it is wrong because of a bug, keeping its place, time and actor.

## Publishing to the registry

Source Loaf archives and compiled units are stored on the GitHub Container Registry as packages linked to this repository, `ghcr.io/encero-systems/incan.pub/crates-io/<name>`; the `index` branch stays the authority and every fetch is verified against it. Credentials reach the tool as `INCAN_PUB_REGISTRY_USER` and `INCAN_PUB_REGISTRY_TOKEN` and reach curl only through a config file.

A package must first be created by a workflow of this repository: it then takes the repository's public visibility. A package whose first push comes from anywhere else is private, there is no API to change that, and an owner has to make it public by hand. Pushes from a maintainer's machine are for packages that already exist.

- **Source archives.** `incan-pub push-source NAME VERSION` pushes one adopted version's source archive under the tag `<version>` (`+` written `_`), rebuilding the archive by adoption when the blob store lacks it and refusing a digest other than the index names. The `publish-sources` workflow runs it for the versions it is given.
- **Compiled units.** The `bake` workflow compiles the shards it is given, each a directory under `bake/` with the root requests (`roots.json`) and the resolution (`lock.json`), in turn on a GitHub-hosted runner of the target's platform (macOS for `aarch64-apple-darwin`, Linux for `x86_64-unknown-linux-gnu`), reusing a unit two shards share. `fetch-sources` pulls the shard's source archives anonymously and `oven compile-closure` compiles them twice, in two roots. `pack-units` packs each unit's store entry (its `artifact.json`, `payload` and the files under `artifacts/`) as one deterministic archive, and with `--reproduces` refuses the run unless both compiles give every unit the same identity, entry and archive bytes. Run the same two compiles locally before dispatching a bake: it takes a minute, where a refused bake is a wasted run. `actions/attest-build-provenance` attests the archives, and `push-units` pushes them, one tag per binding (`<version>-<12 hex of the binding's canonical JSON digest>`), adding layers to those the tag already lists. The run uploads one asset proposal per unit as the `asset-proposals` artifact.
- **Recording facts on a runner.** A build script runs where its target runs, so Linux facts are recorded on Linux. The `record-facts` workflow takes a bindings file under `bake/facts/` (name, version, features, profile, domain, graph per line), runs `incan-pub record-fact --propose` for each in the platform's sandbox (bubblewrap on Linux, `sandbox-exec` on macOS) and uploads the proposals; `incan-pub add-fact` admits them on a checkout of `index`.
- **Recording assets.** `incan-pub admit-assets DIR --work DIR` on a checkout of `index` decides each proposal against the log, pulls its archive anonymously and refuses it unless the bytes arrive with their digest, then appends an `asset` event and rebuilds the projections. An asset needs an adopted record, and one unit identity names one archive. Facts are not checked: a unit's identity carries the facts it compiled under, so a unit compiled without the facts a plan selects never matches that plan. Admitted assets are listed in `crates-io/<name>/<version>/assets.json`.
- **Compilers.** The `toolchain` workflow builds the Incan compiler once per OS at a commit and pushes it to `ghcr.io/encero-systems/incan.pub/toolchain/incan:<commit>-<target triple>`; the other workflows pull it. The `check` workflow publishes the tool the same way, `toolchain/incan-pub:<tool key>-<target triple>` for every push to `main`, keyed by the git hash of what the binary is built from, and `bake` and `publish-sources` pull the one for their commit's key instead of building it. Releases cannot hold it: this repository's releases are immutable once published. It is not relocatable: a job checks out `encero-systems/incan` at the same commit into `incan`, where it was built.

## Correcting a wrong adoption

For the time being, an adoption that is wrong because of a bug is rewritten in place: fix the translation, then run `incan-pub readopt NAME VERSION ... --cache <cache> --blobs <blob store>` against a checkout of `index`. Each named `adopt` event keeps its place, time and actor and takes the corrected payload. A version whose translation comes out the same is right and is refused, so right adoptions never change. Commit, push and report the new `index` HEAD as for any other change.

## Where the contract lives

| | |
|---|---|
| [docs/v0.md](docs/v0.md) | the v0 design: two branches, three programs, admission, what is out of scope |
| [docs/model.md](docs/model.md) | the loop from both ends: what a toolchain does to publish, what the registry does to manage |
| [docs/external-source-record.md](docs/external-source-record.md) | the record, its events, the rendered manifest, the closed vocabulary |
| [docs/harvest-proposal.md](docs/harvest-proposal.md) | what a harvest emits and what `add-fact` admits |
| [encero-systems/incan#1666](https://github.com/encero-systems/incan/issues/1666) | the tracking issue, including the open questions this registry cannot settle alone |
