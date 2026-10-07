# crates-io/brotli-decompressor

The crates.io package [brotli-decompressor](https://crates.io/crates/brotli-decompressor), adopted by [incan.pub](https://github.com/encero-systems/incan.pub) as Loaves. A brotli decompressor that with an interface avoiding the rust stdlib. This makes it suitable for embedded devices and kernels. It is designed with a pluggable allocator so that the standard lib's allocator may be employed. The default build also includes a stdlib allocator and stream interface. Disable this with --features=no-stdlib. Alternatively, --features=unsafe turns off array bounds checks and memory initialization but provides a safe interface for the caller.  Without adding the --features=unsafe argument, all included code is safe. For compression in addition to this library, download https://github.com/dropbox/rust-brotli

| Version | Adopted | License | Build facts | Compiled units | |
|---|---|---|---|---|---|
| [6.0.1](6.0.1/loaf.toml) | 2026-10-07 | BSD-3-Clause/MIT | 0 | 4 | [units](6.0.1/assets.json) |
| [5.0.3](5.0.3/loaf.toml) | 2026-10-06 | BSD-3-Clause/MIT | 1 | 0 |  |

Each version's Loaf carries the package's own LICENSE and README, or ones incan.pub wrote where the package shipped none. Source archives and compiled units are on the GitHub Container Registry as `ghcr.io/encero-systems/incan.pub/crates-io/brotli-decompressor`; a client verifies them against this index: the archive by the `cksum` on its version's line in [index/crates-io/brotli-decompressor](../../index/crates-io/brotli-decompressor), the units by each version's `assets.json`.

<!-- Projection of the incan.pub event log; regenerate with `incan-pub build`, never edit. -->
