# crates-io/cmov

The crates.io package [cmov](https://crates.io/crates/cmov), adopted by [incan.pub](https://github.com/encero-systems/incan.pub) as Loaves. Conditional move CPU intrinsics which are guaranteed on major platforms (ARM32/ARM64, x86/x86_64,
RISC-V) to execute in constant-time and not be rewritten as branches by the compiler. Provides
wrappers for the CMOV family of instructions on x86/x86_64 and CSEL on AArch64, along with a
portable "best-effort" pure Rust fallback implementation.

| Version | Adopted | License | Build facts | Compiled units | |
|---|---|---|---|---|---|
| [0.5.4](0.5.4/loaf.toml) | 2026-10-07 | Apache-2.0 OR MIT | 0 | 4 | [units](0.5.4/assets.json) |

Each version's Loaf carries the package's own LICENSE and README, or ones incan.pub wrote where the package shipped none. Source archives and compiled units are on the GitHub Container Registry as `ghcr.io/encero-systems/incan.pub/crates-io/cmov`; a client verifies them against this index: the archive by the `cksum` on its version's line in [index/crates-io/cmov](../../index/crates-io/cmov), the units by each version's `assets.json`.

<!-- Projection of the incan.pub event log; regenerate with `incan-pub build`, never edit. -->
