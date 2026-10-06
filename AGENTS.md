# Working on incan.pub

This repository is the interim registry of [RFC 125](https://github.com/encero-systems/incan/blob/main/workspaces/docs-site/docs/RFCs/125_incan_pub_loaf_registry_and_baked_asset_distribution.md), in the repository-based v0 shape [docs/v0.md](docs/v0.md) defines. `main` holds the tool and the docs; the orphan [`index`](https://github.com/encero-systems/incan.pub/tree/index) branch holds the data a toolchain reads and pins by commit.

## Rules

1. **The tool is written in Incan.** Not Python, not shell beyond the two CI helpers. A registry whose admission were written in another language would be a claim the language cannot make about itself.
2. **Projections are never edited.** Change the log under `events/`, run `incan-pub build`, commit both. `incan-pub check` refuses a checkout whose events, committed generated inputs or projections disagree, and CI runs it on both branches.
3. **The fact vocabulary is closed** and mirrors the compiler's `RustFactRecord`; [docs/external-source-record.md](docs/external-source-record.md) states what a key entering it must satisfy. Adding a key here without the compiler reading it publishes a fact no consumer can apply.
4. **The tool is built on the Incan dev line.** No release installs it yet, so CI builds the compiler at the commit `.github/workflows/check.yml` pins, and so does a developer machine. A compiler gap is worked around in place with a comment naming it, and reported to the dev line rather than patched here.

## Building

```sh
INCAN=/path/to/incan ./build.sh   # lock, bake, test, build -> target/incan-pub
```

`INCAN` is a compiler built from the commit CI pins (`cargo build -p incan-cli` in a checkout of encero-systems/incan, debug, with its own Cargo target directory); the `incan` on `PATH` when unset. Any source change needs a fresh `incan oven bake --project .` before `incan check` accepts the project again, and `build.sh` does that. `build.sh` takes its own `INCAN_HOME` under `target/`. The default `~/.incan` is shared with every toolchain on the machine, and a store entry written by another one reads here as "Oven store integrity failure: manifest identity does not match its immutable content". Override `INCAN_HOME` only to reuse a warm store deliberately. The same applies to any Cargo target directory: a compiler binary embeds its repository root, so two checkouts building into one target poison each other.

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

Adoption turns a crates.io package version into a Loaf ([RFC 125](https://github.com/encero-systems/incan/blob/main/workspaces/docs-site/docs/RFCs/125_incan_pub_loaf_registry_and_baked_asset_distribution.md), "Adoption from crates.io"): one `adopt` event per version, whose manifest follows RFC 119's Rust facet grammar and whose source Loaf archive is stored by digest. Admission refuses an `adopt` event whose dependencies are not adopted yet, optional and target-conditional ones included, so `adopt` plans the whole closure first and appends in dependency order. Against a checkout of `index`:

```sh
incan-pub adopt --records --cache <cache> --blobs <blob store>   # or: adopt NAME VERSION ...
incan-pub check --crates-io-index <snapshot> --blobs <blob store>
git commit && git push
```

`--cache` keeps the crates.io index files and `.crate` archives adoption downloads, each checked against its crates.io checksum before it is read; `--blobs` receives every source Loaf archive as `sha256/<hex>.tar`. A requirement no adopted version meets is met by the newest crates.io version satisfying it, preferring one that is not yanked. Report the resulting `index` HEAD as for a harvest.

## Where the contract lives

| | |
|---|---|
| [docs/v0.md](docs/v0.md) | the v0 design: two branches, three programs, admission, what is out of scope |
| [docs/model.md](docs/model.md) | the loop from both ends: what a toolchain does to publish, what the registry does to manage |
| [docs/external-source-record.md](docs/external-source-record.md) | the record, its events, the rendered manifest, the closed vocabulary |
| [docs/harvest-proposal.md](docs/harvest-proposal.md) | what a harvest emits and what `add-fact` admits |
| [encero-systems/incan#1666](https://github.com/encero-systems/incan/issues/1666) | the tracking issue, including the open questions this registry cannot settle alone |
