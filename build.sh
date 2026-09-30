#!/usr/bin/env sh
# Test incan-pub, build it with the installed Incan toolchain and place the binary at target/incan-pub.
#
# `incan build --report json` names the binary it published as the artifact of kind `binary`; this copies it to one
# stable location so CI and the index branch's workflow can call `target/incan-pub` without knowing the Oven output
# layout.
set -eu
cd "$(dirname "$0")"
export INCAN_NO_BANNER=1
# Own Oven home. The default `~/.incan` is shared with every other toolchain on the machine, and a store entry
# written by one toolchain is an integrity failure to another ("manifest identity does not match its immutable
# content"); the same lesson as a shared Cargo target. Override INCAN_HOME to reuse a warm store deliberately.
export INCAN_HOME="${INCAN_HOME:-$PWD/target/incan-home}"
mkdir -p "$INCAN_HOME"
# The 0.5.1 toolchain lowers a negated parenthesized `and` to a constant false, so `if not (A and B):` never runs
# the body it guards — a validator written that way silently stops validating, and only a test asserting the exact
# refusal notices. Correct on 0.6 (encero-systems/incan#1966); refused here until this tool moves off 0.5.1.
if grep -rn 'not ([^)]* and ' src tests; then
  printf 'build.sh: `not (A and B)` above is mis-lowered to a constant false by the 0.5.1 toolchain; write it as nested ifs or `not A or not B`\n' >&2
  exit 1
fi
# A fresh checkout has no project inspection authority yet: the bake establishes it for the `rust::` imports in
# src/host.incn, then test and build reuse the sealed Loaf.
incan oven bake --project .
incan test
mkdir -p target
incan build src/main.incn --report json --report-output target/build-report.json
binary="$(tr -d '\n' < target/build-report.json | sed -n 's/.*"kind":[[:space:]]*"binary",[[:space:]]*"path":[[:space:]]*"\([^"]*\)".*/\1/p')"
[ -n "$binary" ] && [ -x "$binary" ] || {
  printf 'build.sh: the build report names no executable binary artifact (see target/build-report.json)\n' >&2
  exit 1
}
cp "$binary" target/incan-pub
printf 'built target/incan-pub\n'
