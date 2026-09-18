#!/usr/bin/env sh
# Snapshot the crates.io sparse-index files that the log names, for `incan-pub check --crates-io-index`.
#
# The tool opens no socket; this is the only network step of a check. It asks the tool which packages the log names
# (`incan-pub index-urls`), fetches each package's index file from index.crates.io and lays them out as the sparse
# index does, so the snapshot directory reads like a checkout of the index.
#
#   snapshot-crates-io-index.sh <incan-pub binary> <snapshot directory> [--root <index checkout>]
set -eu
tool="$1"
snapshot="$2"
shift 2
mkdir -p "$snapshot"
"$tool" index-urls "$@" | while IFS="$(printf '\t')" read -r name path url; do
  mkdir -p "$snapshot/$(dirname "$path")"
  curl -fsSL "$url" -o "$snapshot/$path"
  printf 'snapshot: %s\n' "$name"
done
