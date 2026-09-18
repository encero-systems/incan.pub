#!/usr/bin/env sh
# Test incan-pub, build it with the installed Incan toolchain and place the binary at target/incan-pub.
#
# `incan build --report json` names the binary it published as the artifact of kind `binary`; this copies it to one
# stable location so CI and the index branch's workflow can call `target/incan-pub` without knowing the Oven output
# layout.
set -eu
cd "$(dirname "$0")"
export INCAN_NO_BANNER=1
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
