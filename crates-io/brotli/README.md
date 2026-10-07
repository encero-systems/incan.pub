# crates-io/brotli

The crates.io package [brotli](https://crates.io/crates/brotli), adopted by [incan.pub](https://github.com/encero-systems/incan.pub) as Loaves. A brotli compressor and decompressor that with an interface avoiding the rust stdlib. This makes it suitable for embedded devices and kernels. It is designed with a pluggable allocator so that the standard lib's allocator may be employed. The default build also includes a stdlib allocator and stream interface. Disable this with --features=no-stdlib. All included code is safe.

| Version | Adopted | License | Build facts | Compiled units | |
|---|---|---|---|---|---|
| [9.0.0](9.0.0/loaf.toml) | 2026-10-07 | BSD-3-Clause AND MIT | 0 | 2 | [units](9.0.0/assets.json) |
| [8.0.4](8.0.4/loaf.toml) | 2026-10-06 | BSD-3-Clause AND MIT | 1 | 0 |  |

Each version's Loaf carries the package's own LICENSE and README, or ones incan.pub wrote where the package shipped none. Source archives and compiled units are on the GitHub Container Registry as `ghcr.io/encero-systems/incan.pub/crates-io/brotli`; a client verifies them against this index: the archive by the `cksum` on its version's line in [index/crates-io/brotli](../../index/crates-io/brotli), the units by each version's `assets.json`.

<!-- Projection of the incan.pub event log; regenerate with `incan-pub build`, never edit. -->
