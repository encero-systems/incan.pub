# crates-io/ipnet

The crates.io package [ipnet](https://crates.io/crates/ipnet), adopted by [incan.pub](https://github.com/encero-systems/incan.pub) as Loaves. Provides types and useful methods for working with IPv4 and IPv6 network addresses, commonly called IP prefixes. The new `IpNet`, `Ipv4Net`, and `Ipv6Net` types build on the existing `IpAddr`, `Ipv4Addr`, and `Ipv6Addr` types already provided in Rust's standard library and align to their design to stay consistent. The module also provides useful traits that extend `Ipv4Addr` and `Ipv6Addr` with methods for `Add`, `Sub`, `BitAnd`, and `BitOr` operations. The module only uses stable feature so it is guaranteed to compile using the stable toolchain.

| Version | Adopted | License | Build facts | Compiled units | |
|---|---|---|---|---|---|
| [2.12.2](2.12.2/loaf.toml) | 2026-10-07 | MIT OR Apache-2.0 | 0 | 0 |  |
| [2.12.0](2.12.0/loaf.toml) | 2026-10-07 | MIT OR Apache-2.0 | 0 | 0 |  |

Each version's Loaf carries the package's own LICENSE and README, or ones incan.pub wrote where the package shipped none. Source archives and compiled units are on the GitHub Container Registry as `ghcr.io/encero-systems/incan.pub/crates-io/ipnet`; a client verifies them against this index: the archive by the `cksum` on its version's line in [index/crates-io/ipnet](../../index/crates-io/ipnet), the units by each version's `assets.json`.

<!-- Projection of the incan.pub event log; regenerate with `incan-pub build`, never edit. -->
