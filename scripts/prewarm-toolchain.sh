#!/usr/bin/env sh
# Prefetch the crates the 0.5.1 toolchain's test envelope needs, so a fresh machine can `incan test`.
#
# On a fresh install, `incan oven bake --project .` for a project with a `tests/` directory publishes its test
# dependency envelope through Oven's Cargo-backed compatibility publisher, which runs `--offline`. The envelope's
# lock reaches crates (anyhow, through the toolchain's own workspace lock) that the release Loaf ships as sources but
# not in the local Cargo registry, so the publisher fails with "failed to download `anyhow`". Warming the registry
# from the toolchain's workspace manifest is the smallest repair; see encero-systems/incan#1667. On a developer
# machine with a used Cargo registry this is a no-op.
set -eu
incan_home="${INCAN_HOME:-$HOME/.incan}"
version="$(incan --version | sed -n 's/^incan //p')"
manifest="$incan_home/toolchains/$version/crates/Cargo.toml"
[ -f "$manifest" ] || { printf 'prewarm-toolchain: no toolchain workspace at %s\n' "$manifest" >&2; exit 1; }
cargo="$(ls -d "$incan_home"/rust/toolchains/*/bin/cargo 2>/dev/null | head -n 1)"
[ -x "$cargo" ] || { printf 'prewarm-toolchain: no Incan-owned cargo under %s/rust/toolchains\n' "$incan_home" >&2; exit 1; }
RUSTUP_HOME="$incan_home/rust" "$cargo" fetch --manifest-path "$manifest"
printf 'prewarm-toolchain: registry warmed for incan %s\n' "$version"
