#!/usr/bin/env sh
# Test loaves-pub, build it with an installed Incan toolchain and place the binary at target/loaves-pub, one stable
# location CI and the index branch's workflow can call without knowing Oven's output layout.
#
# `INCAN` selects the toolchain's `incan`; the one on PATH when unset. INCAN_HOME names its installation and
# INCAN_STDLIB the standard library sources of that release.
set -eu
cd "$(dirname "$0")"
export INCAN_NO_BANNER=1
INCAN="${INCAN:-incan}"
for tests in tests/test_*.incn; do
  "$INCAN" test "$tests"
done
"$INCAN" build src/main.incn --profile release
binary=target/incan/release/bin/loaves_pub
[ -x "$binary" ] || {
  printf 'build.sh: the build left no executable at %s\n' "$binary" >&2
  exit 1
}
# Replace the file rather than write into it: on macOS a running process whose executable is rewritten in place is
# killed (its code signature no longer matches), so an admission running target/loaves-pub would die mid-write.
rm -f target/loaves-pub
cp "$binary" target/loaves-pub
printf 'built target/loaves-pub\n'
