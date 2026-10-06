#!/usr/bin/env sh
# Test incan-pub, build it with an Incan toolchain and place the binary at target/incan-pub.
#
# `incan build --report json` names the binary it published as the artifact of kind `binary`; this copies it to one
# stable location so CI and the index branch's workflow can call `target/incan-pub` without knowing the Oven output
# layout. `INCAN` selects the toolchain binary; the one on PATH when unset.
set -eu
cd "$(dirname "$0")"
export INCAN_NO_BANNER=1
INCAN="${INCAN:-incan}"
# Own Oven home. The default `~/.incan` is shared with every other toolchain on the machine, and a store entry
# written by one toolchain is an integrity failure to another ("manifest identity does not match its immutable
# content"); the same lesson as a shared Cargo target. Override INCAN_HOME to reuse a warm store deliberately.
export INCAN_HOME="${INCAN_HOME:-$PWD/target/incan-home}"
mkdir -p "$INCAN_HOME"
# The bake establishes the project inspection authority for the `rust::` imports in src/host.incn; check, test and
# build then reuse the sealed Loaf. Any source change needs a fresh bake on this toolchain.
"$INCAN" lock
"$INCAN" oven bake --project .
"$INCAN" test
mkdir -p target
"$INCAN" build src/main.incn --report json --report-output target/build-report.json
binary="$(tr -d '\n' < target/build-report.json | sed -n 's/.*"kind":[[:space:]]*"binary",[[:space:]]*"path":[[:space:]]*"\([^"]*\)".*/\1/p')"
[ -n "$binary" ] && [ -x "$binary" ] || {
  printf 'build.sh: the build report names no executable binary artifact (see target/build-report.json)\n' >&2
  exit 1
}
cp "$binary" target/incan-pub
printf 'built target/incan-pub\n'
