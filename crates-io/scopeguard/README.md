# crates-io/scopeguard

The crates.io package [scopeguard](https://crates.io/crates/scopeguard), adopted by [incan.pub](https://github.com/encero-systems/incan.pub) as Loaves. A RAII scope guard that will run a given closure when it goes out of scope,
even if the code between panics (assuming unwinding panic).

Defines the macros `defer!`, `defer_on_unwind!`, `defer_on_success!` as
shorthands for guards with one of the implemented strategies.

| Version | Adopted | License | Build facts | Compiled units | |
|---|---|---|---|---|---|
| [1.2.0](1.2.0/loaf.toml) | 2026-10-06 | MIT OR Apache-2.0 | 1 | 8 | [units](1.2.0/assets.json) |

Each version's Loaf carries the package's own LICENSE and README, or ones incan.pub wrote where the package shipped none. Source archives and compiled units are on the GitHub Container Registry as `ghcr.io/encero-systems/incan.pub/crates-io/scopeguard`; a client verifies them against this index: the archive by the `cksum` on its version's line in [index/crates-io/scopeguard](../../index/crates-io/scopeguard), the units by each version's `assets.json`.

<!-- Projection of the incan.pub event log; regenerate with `incan-pub build`, never edit. -->
