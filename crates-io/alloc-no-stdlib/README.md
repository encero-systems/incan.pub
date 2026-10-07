# crates-io/alloc-no-stdlib

The crates.io package [alloc-no-stdlib](https://crates.io/crates/alloc-no-stdlib), adopted by [incan.pub](https://github.com/encero-systems/incan.pub) as Loaves. A dynamic allocator that may be used with or without the stdlib. This allows a package with nostd to allocate memory dynamically and be used either with a custom allocator, items on the stack, or by a package that wishes to simply use Box<>. It also provides options to use calloc or a mutable global variable for pre-zeroed memory

| Version | Adopted | License | Build facts | Compiled units | |
|---|---|---|---|---|---|
| [3.0.0](3.0.0/loaf.toml) | 2026-10-07 | BSD-3-Clause | 0 | 0 |  |
| [2.0.4](2.0.4/loaf.toml) | 2026-10-06 | BSD-3-Clause | 1 | 0 |  |

Each version's Loaf carries the package's own LICENSE and README, or ones incan.pub wrote where the package shipped none. Source archives and compiled units are on the GitHub Container Registry as `ghcr.io/encero-systems/incan.pub/crates-io/alloc-no-stdlib`; a client verifies them against this index: the archive by the `cksum` on its version's line in [index/crates-io/alloc-no-stdlib](../../index/crates-io/alloc-no-stdlib), the units by each version's `assets.json`.

<!-- Projection of the incan.pub event log; regenerate with `incan-pub build`, never edit. -->
