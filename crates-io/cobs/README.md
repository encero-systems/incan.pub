# crates-io/cobs

The crates.io package [cobs](https://crates.io/crates/cobs), adopted by [incan.pub](https://github.com/encero-systems/incan.pub) as Loaves. This is an implementation of the Consistent Overhead Byte Stuffing (COBS) algorithm.
    COBS is an algorithm for transforming a message into an encoding where a specific value
    (the "sentinel" value) is not used. This value can then be used to mark frame boundaries
    in a serial communication channel.

| Version | Adopted | License | Build facts | Compiled units | |
|---|---|---|---|---|---|
| [0.5.1](0.5.1/loaf.toml) | 2026-10-07 | MIT OR Apache-2.0 | 0 | 1 | [units](0.5.1/assets.json) |
| [0.3.0](0.3.0/loaf.toml) | 2026-10-07 | MIT OR Apache-2.0 | 0 | 0 |  |

Each version's Loaf carries the package's own LICENSE and README, or ones incan.pub wrote where the package shipped none. Source archives and compiled units are on the GitHub Container Registry as `ghcr.io/encero-systems/incan.pub/crates-io/cobs`; a client verifies them against this index: the archive by the `cksum` on its version's line in [index/crates-io/cobs](../../index/crates-io/cobs), the units by each version's `assets.json`.

<!-- Projection of the incan.pub event log; regenerate with `incan-pub build`, never edit. -->
