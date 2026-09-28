# Working on incan.pub

This repository is the interim registry of [RFC 125](https://github.com/encero-systems/incan/blob/main/workspaces/docs-site/docs/RFCs/125_incan_pub_loaf_registry_and_baked_asset_distribution.md), in the repository-based v0 shape [docs/v0.md](docs/v0.md) defines. `main` holds the tool and the docs; the orphan [`index`](https://github.com/encero-systems/incan.pub/tree/index) branch holds the data a toolchain reads and pins by commit.

## Rules

1. **The tool is written in Incan.** Not Python, not shell beyond the two CI helpers. A registry whose admission were written in another language would be a claim the language cannot make about itself.
2. **Projections are never edited.** Change the log under `events/`, run `incan-pub build`, commit both. `incan-pub check` refuses a checkout whose events, committed generated inputs or projections disagree, and CI runs it on both branches.
3. **The fact vocabulary is closed** and mirrors the compiler's `RustFactRecord`; [docs/external-source-record.md](docs/external-source-record.md) states what a key entering it must satisfy. Adding a key here without the compiler reading it publishes a fact no consumer can apply.
4. **Nothing 0.5-era is fixed.** The tool is built by the released 0.5.1 toolchain only because it is the installable one; what 0.5.1 lacks is scaffolded around and marked, and comes out when a 0.6 toolchain can be installed from a release manifest. See the last two sections of [docs/v0.md](docs/v0.md).

## Building

```sh
./scripts/prewarm-toolchain.sh   # once per fresh toolchain install (encero-systems/incan#1667)
./build.sh                       # bake, test, build -> target/incan-pub
```

`build.sh` takes its own `INCAN_HOME` under `target/`. The default `~/.incan` is shared with every toolchain on the machine, and a store entry written by another one reads here as "Oven store integrity failure: manifest identity does not match its immutable content". Override `INCAN_HOME` only to reuse a warm store deliberately. The same applies to any Cargo target directory: a compiler binary embeds its repository root, so two checkouts building into one target poison each other.

## Admitting a harvest

A harvest drop is a directory of `<name>-<version>-<profile>/proposal.json` plus the `out/` files each proposal names, produced by the release bake ([docs/harvest-proposal.md](docs/harvest-proposal.md) is the contract). Against a checkout of `index`:

```sh
for p in <drop>/*/; do incan-pub add-fact "$p/proposal.json" --publish; done
incan-pub build
incan-pub check --crates-io-index <snapshot>     # scripts/snapshot-crates-io-index.sh writes the snapshot
git commit && git push
```

Then report the resulting `index` HEAD: that commit is what a toolchain manifest pins and what the next release bake consumes. A proposal admission refuses goes back to whoever harvested it verbatim — the fix belongs on the harvest side, never in the proposal or the log. An identical binding is a no-op; a different one for a binding already on record is a refusal, and that refusal is the point: it is how a record made by hand gets checked against the machine.

Records the harvest declines to propose (a script whose emitted environment it cannot prove unread, reserved `link`/`tool` work) keep whatever record they have. Do not hand-write a fact to fill the gap.

## Where the contract lives

| | |
|---|---|
| [docs/v0.md](docs/v0.md) | the v0 design: two branches, three programs, admission, what is out of scope |
| [docs/model.md](docs/model.md) | the loop from both ends: what a toolchain does to publish, what the registry does to manage |
| [docs/external-source-record.md](docs/external-source-record.md) | the record, its events, the rendered manifest, the closed vocabulary |
| [docs/harvest-proposal.md](docs/harvest-proposal.md) | what a harvest emits and what `add-fact` admits |
| [encero-systems/incan#1666](https://github.com/encero-systems/incan/issues/1666) | the tracking issue, including the open questions this registry cannot settle alone |
