# crates-io/wnaf

The crates.io package [wnaf](https://crates.io/crates/wnaf), adopted by [incan.pub](https://github.com/encero-systems/incan.pub) as Loaves. wNAF (w-ary non-adjacent form) variable-time scalar multiplication implemented generically
over elliptic curve groups with full `no_alloc` support, including multiscalar multiplication using
Straus's interleaved window method. Contains an implementation adapted from the `group` crate which
has been forked and enhanced but is otherwise compatible with that crate's traits, along with the
`ff` crate's traits for finite field representations

| Version | Adopted | License | Build facts | Compiled units | |
|---|---|---|---|---|---|
| [0.14.1](0.14.1/loaf.toml) | 2026-10-07 | Apache-2.0 OR MIT | 0 | 4 | [units](0.14.1/assets.json) |

Each version's Loaf carries the package's own LICENSE and README, or ones incan.pub wrote where the package shipped none. Source archives and compiled units are on the GitHub Container Registry as `ghcr.io/encero-systems/incan.pub/crates-io/wnaf`; a client verifies them against this index: the archive by the `cksum` on its version's line in [index/crates-io/wnaf](../../index/crates-io/wnaf), the units by each version's `assets.json`.

<!-- Projection of the incan.pub event log; regenerate with `incan-pub build`, never edit. -->
