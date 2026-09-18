# incan.pub — `index`

The data branch of [encero-systems/incan.pub](https://github.com/encero-systems/incan.pub): the append-only event
log and the projections folded from it. The tool that admits events and renders projections lives on `main`; this
branch holds only what it manages. A checkout of this branch is what the Incan toolchain reads as its registry
provider in v0 (see `docs/v0.md` on `main`).

```
events/NNNNNN-<kind>-<subject>.json    the append-only event log; the only thing that is authored
crates-io/<name>/<version>/loaf.toml   projection: the external-source record as a consumer reads it
crates-io/<name>/<version>/out/...     committed generated inputs a record names
index/...                              projection: static sparse index, one JSON line per package version
```

Nothing under `crates-io/**/loaf.toml` or `index/` is edited by hand. `incan-pub build` regenerates both from the
events; `incan-pub check`, run in CI on every push, refuses a branch whose events, committed generated inputs or
projections disagree, and verifies every record's checksum against a snapshot of the crates.io sparse index.

A commit is the signed event; `HEAD` is the checkpoint the toolchain pins.
