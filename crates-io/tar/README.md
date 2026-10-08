# crates-io/tar

The crates.io package [tar](https://crates.io/crates/tar), adopted by [incan.pub](https://github.com/encero-systems/incan.pub) as Loaves. A Rust implementation of a TAR file reader and writer. This library does not
currently handle compression, but it is abstract over all I/O readers and
writers. Additionally, great lengths are taken to ensure that the entire
contents are never required to be entirely resident in memory all at once.

| Version | Adopted | License | Build facts | Compiled units | |
|---|---|---|---|---|---|
| [0.4.46](0.4.46/loaf.toml) | 2026-10-07 | MIT OR Apache-2.0 | 0 | 3 | [units](0.4.46/assets.json) |

Each version's Loaf carries the package's own LICENSE and README, or ones incan.pub wrote where the package shipped none. Source archives and compiled units are on the GitHub Container Registry as `ghcr.io/encero-systems/incan.pub/crates-io/tar`; a client verifies them against this index: the archive by the `cksum` on its version's line in [index/crates-io/tar](../../index/crates-io/tar), the units by each version's `assets.json`.

<!-- Projection of the incan.pub event log; regenerate with `incan-pub build`, never edit. -->
