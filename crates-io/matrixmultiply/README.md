# crates-io/matrixmultiply

The crates.io package [matrixmultiply](https://crates.io/crates/matrixmultiply), adopted by [incan.pub](https://github.com/encero-systems/incan.pub) as Loaves. General matrix multiplication for f32 and f64 matrices. Operates on matrices with general layout (they can use arbitrary row and column stride). Detects and uses SIMD features on x86/x86-64 and AArch64 transparently for higher performance. Uses a microkernel strategy, so that the implementation is easy to parallelize and optimize.

Supports multithreading.

| Version | Adopted | License | Build facts | Compiled units | |
|---|---|---|---|---|---|
| [0.3.11](0.3.11/loaf.toml) | 2026-10-07 | MIT/Apache-2.0 | 6 | 4 | [units](0.3.11/assets.json) |

Each version's Loaf carries the package's own LICENSE and README, or ones incan.pub wrote where the package shipped none. Source archives and compiled units are on the GitHub Container Registry as `ghcr.io/encero-systems/incan.pub/crates-io/matrixmultiply`; a client verifies them against this index: the archive by the `cksum` on its version's line in [index/crates-io/matrixmultiply](../../index/crates-io/matrixmultiply), the units by each version's `assets.json`.

<!-- Projection of the incan.pub event log; regenerate with `incan-pub build`, never edit. -->
