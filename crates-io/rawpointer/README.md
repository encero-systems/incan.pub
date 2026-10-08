# crates-io/rawpointer

The crates.io package [rawpointer](https://crates.io/crates/rawpointer), adopted by [incan.pub](https://github.com/encero-systems/incan.pub) as Loaves. Extra methods for raw pointers and `NonNull<T>`.

For example `.post_inc()` and `.pre_dec()` (c.f. `ptr++` and `--ptr`),
`offset` and `add` for `NonNull<T>`, and the function `ptrdistance`.

| Version | Adopted | License | Build facts | Compiled units | |
|---|---|---|---|---|---|
| [0.2.1](0.2.1/loaf.toml) | 2026-10-07 | MIT/Apache-2.0 | 0 | 6 | [units](0.2.1/assets.json) |

Each version's Loaf carries the package's own LICENSE and README, or ones incan.pub wrote where the package shipped none. Source archives and compiled units are on the GitHub Container Registry as `ghcr.io/encero-systems/incan.pub/crates-io/rawpointer`; a client verifies them against this index: the archive by the `cksum` on its version's line in [index/crates-io/rawpointer](../../index/crates-io/rawpointer), the units by each version's `assets.json`.

<!-- Projection of the incan.pub event log; regenerate with `incan-pub build`, never edit. -->
