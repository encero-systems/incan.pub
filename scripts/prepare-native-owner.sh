#!/bin/bash
set -euo pipefail
[ "$TARGET" = x86_64-unknown-linux-gnu ] || { echo "::error::native owner preparation requires Linux"; exit 1; }
. /etc/os-release
[ "$ID" = ubuntu ] && [ "$VERSION_ID" = 24.04 ] || { echo "::error::native owner requires Ubuntu 24.04"; exit 1; }
case "$NATIVE_OWNER" in bake/facts/*/native-owner.json) ;; *) echo "::error::invalid native owner manifest path"; exit 1;; esac
tool="$(realpath tool)"
manifest="$(realpath "tool/$NATIVE_OWNER")"
case "$manifest" in "$tool"/bake/facts/*/native-owner.json) ;; *) echo "::error::native owner manifest escaped checked-in recipes"; exit 1;; esac
jq -e '.schema == "incan.review.linux-header-installation/1" and .owner == "/usr" and
  (.compiler_sha256 | test("^[0-9a-f]{64}$")) and (.ar_sha256 | test("^[0-9a-f]{64}$")) and
  (.header_inventory_sha256 | test("^[0-9a-f]{64}$")) and .header_count > 0' "$manifest" > /dev/null
root="$(jq -r '.root' "$manifest")"
case "$root" in /usr/local/share/incan-native-sysroots/*) ;; *) echo "::error::unconfined native header root"; exit 1;; esac
[ "$(realpath -m "$root")" = "$root" ] || { echo "::error::noncanonical native header root"; exit 1; }
# Extract exact package inputs into a private owner; never replace the runner's system packages.
staging="$RUNNER_TEMP/native-package-inputs"
owner="$RUNNER_TEMP/native-owner"
headers="$RUNNER_TEMP/native-header-inputs"
mkdir "$staging" "$owner" "$headers"
jq -e '.transport_archives | type == "array" and length > 0 and
  ([.[].package] == ([.[].package] | sort | unique)) and
  all(.[]; (.package | test("^[a-z0-9][a-z0-9+.:_-]*$")) and
    (.version | test("^[a-zA-Z0-9.+:~_-]+$")) and .architecture == "amd64" and
    (.url | test("^https://snapshot\\.ubuntu\\.com/ubuntu/[0-9]{8}T[0-9]{6}Z/pool/[A-Za-z0-9/_.%+~-]+\\.deb$")) and
    (.sha256 | test("^[0-9a-f]{64}$")))' "$manifest" > /dev/null
jq -r '.transport_archives[] | [.package, .version, .architecture, .url, .sha256] | @tsv' "$manifest" > "$RUNNER_TEMP/native-packages.txt"
while IFS=$'\t' read -r package version architecture url expected; do
  download="$RUNNER_TEMP/package-${package%%:*}"
  mkdir "$download"
  archive="$download/input.deb"
  curl --fail --location --proto '=https' --retry 3 "$url" -o "$archive"
  [ "$(sha256sum "$archive" | cut -d ' ' -f1)" = "$expected" ] || { echo "::error::package archive digest differs"; exit 1; }
  [ "$(dpkg-deb -f "$archive" Package)" = "${package%%:*}" ] && \
    [ "$(dpkg-deb -f "$archive" Version)" = "$version" ] && \
    [ "$(dpkg-deb -f "$archive" Architecture)" = "$architecture" ] \
    || { echo "::error::downloaded package identity differs"; exit 1; }
  dpkg-deb -x "$archive" "$staging"
  case "$package" in libc6-dev:amd64|linux-libc-dev:amd64) dpkg-deb -x "$archive" "$headers";; esac
done < "$RUNNER_TEMP/native-packages.txt"
compiler_source="$(jq -r '.compiler' "$manifest")"
resource_source="$(jq -r '.resource' "$manifest")"
for path in "$compiler_source" "$resource_source"; do
  case "$path" in /usr/*) ;; *) echo "::error::compiler input escapes native owner"; exit 1;; esac
  [ "$(realpath -e "$staging$path")" = "$staging$path" ] || { echo "::error::noncanonical compiler package input"; exit 1; }
done
[ -f "$staging$compiler_source" ] && [ ! -L "$staging$compiler_source" ]
[ "$(sha256sum "$staging$compiler_source" | cut -d ' ' -f1)" = "$(jq -r '.compiler_sha256' "$manifest")" ] || { echo "::error::compiler bytes differ"; exit 1; }
compiler="$owner/${compiler_source#/usr/}"
resource="$owner/${resource_source#/usr/}"
mkdir -p "$(dirname "$compiler")" "$(dirname "$resource")"
install -m 0555 "$staging$compiler_source" "$compiler"
cp -a "$staging$resource_source" "$resource"
cc="$compiler"
cxx="$compiler"
ar_source="$(realpath -e "$staging$(jq -r '.ar' "$manifest")")"
case "$ar_source" in "$staging"/usr/*) ;; *) echo "::error::archiver escapes package inputs"; exit 1;; esac
[ "$(sha256sum "$ar_source" | cut -d ' ' -f1)" = "$(jq -r '.ar_sha256' "$manifest")" ] || { echo "::error::archiver bytes differ"; exit 1; }
ar="$owner/bin/$(basename "$ar_source")"
mkdir -p "$(dirname "$ar")"
install -m 0555 "$ar_source" "$ar"
root="$owner/${root#/usr/}"
# Exact loader files are copied under their observed SONAMEs, without symlinks or ambient library mounts.
if jq -e 'has("runtime_files")' "$manifest" > /dev/null; then
  runtime_source_root="$(jq -r '.runtime_root' "$manifest")"
  runtime_root="$runtime_source_root"
  case "$runtime_root" in /usr/local/share/incan-native-runtimes/*) ;; *) echo "::error::unconfined runtime root"; exit 1;; esac
  [ "$(realpath -m "$runtime_root")" = "$runtime_root" ]
  runtime_root="$owner/${runtime_root#/usr/}"
  [ ! -e "$runtime_root" ] && [ ! -L "$runtime_root" ] || { echo "::error::runtime root must be fresh"; exit 1; }
  jq -e '.runtime_files | type == "array" and length > 0 and
    ([.[].name] == ([.[].name] | sort | unique)) and
    all(.[]; (.name != "." and .name != "..") and (.name | test("^[A-Za-z0-9_.+-]+$")) and
      (.source | startswith("/usr/")) and (.sha256 | test("^[0-9a-f]{64}$")) and (.size > 0))' "$manifest" > /dev/null
  jq -r '.runtime_files[] | [.name, .source, .sha256, .size] | @tsv' "$manifest" > "$RUNNER_TEMP/native-runtime.tsv"
  while IFS=$'\t' read -r name source expected bytes; do
    source="$staging$source"
    [ -f "$source" ] && [ ! -L "$source" ] && [ "$(realpath -e "$source")" = "$source" ]
    [ "$(sha256sum "$source" | cut -d ' ' -f1)" = "$expected" ] && [ "$(stat -c '%s' "$source")" = "$bytes" ] \
      || { echo "::error::native runtime input differs: $source"; exit 1; }
  done < "$RUNNER_TEMP/native-runtime.tsv"
  mkdir -p "$(dirname "$runtime_root")"
  mkdir "$runtime_root"
  while IFS=$'\t' read -r name source expected bytes; do
    destination="$runtime_root/$name"
    install -m 0555 "$staging$source" "$destination"
    [ -f "$destination" ] && [ ! -L "$destination" ]
    [ "$(realpath -e "$destination")" = "$destination" ]
    [ "$(sha256sum "$destination" | cut -d ' ' -f1)" = "$expected" ] && [ "$(stat -c '%s' "$destination")" = "$bytes" ] \
      || { echo "::error::installed runtime bytes differ: $name"; exit 1; }
  done < "$RUNNER_TEMP/native-runtime.tsv"
  jq --arg root "$runtime_root" '{runtime_files: [.runtime_files[].name | $root + "/" + .] | sort}' "$manifest" > "$RUNNER_TEMP/native-runtime-files.json"
fi
[ ! -e "$root" ] && [ ! -L "$root" ] || { echo "::error::native root must be fresh and task-owned"; exit 1; }
rows="$RUNNER_TEMP/native-header-rows.jsonl"
: > "$rows"
find "$headers/usr/include" \( -type f -o -type l \) -print | sed "s|^$headers||" | LC_ALL=C sort -u > "$RUNNER_TEMP/native-header-paths.txt"
while IFS= read -r source; do
  case "$source" in /usr/include/*) ;; *) continue;; esac
  resolved="$(realpath -e "$headers$source")"
  case "$resolved" in "$headers"/usr/include/*) ;; *) echo "::error::header escapes installed package inputs"; exit 1;; esac
  [ -f "$resolved" ] || { echo "::error::header is not a regular file"; exit 1; }
  digest="$(sha256sum "$resolved" | cut -d ' ' -f1)"
  bytes="$(stat -c '%s' "$resolved")"
  jq -nc --arg path "$source" --arg resolved "${resolved#"$headers"}" --arg digest "$digest" --argjson bytes "$bytes" \
    '{path: $path, resolved_source: $resolved, sha256: $digest, bytes: $bytes}' >> "$rows"
done < "$RUNNER_TEMP/native-header-paths.txt"
jq -scS '.' "$rows" > "$RUNNER_TEMP/native-headers.json"
digest="$(jq -jcS '.' "$RUNNER_TEMP/native-headers.json" | sha256sum | cut -d ' ' -f1)"
count="$(jq 'length' "$RUNNER_TEMP/native-headers.json")"
[ "$digest" = "$(jq -r '.header_inventory_sha256' "$manifest")" ] && [ "$count" = "$(jq -r '.header_count' "$manifest")" ] \
  || { echo "::error::actual package header inventory differs"; exit 1; }
jq -S 'del(.root, .owner, .cc, .cxx, .ar, .ar_sha256, .runtime_root, .runtime_files, .transport_package_versions, .transport_archives)' "$manifest" > "$RUNNER_TEMP/native-toolchain.json"
# Admission happens before installation; a fresh root cannot contain unrecorded headers.
mkdir -p "$(dirname "$root")"
mkdir "$root"
while IFS= read -r source; do
  resolved="$(realpath -e "$headers$source")"
  case "$resolved" in "$headers"/usr/include/*) ;; *) echo "::error::header changed outside extracted inputs"; exit 1;; esac
  install -D -m 0444 "$resolved" "$root$source"
done < "$RUNNER_TEMP/native-header-paths.txt"
install -m 0444 "$RUNNER_TEMP/native-toolchain.json" "$root/incan-toolchain.json"
{ cat "$RUNNER_TEMP/native-header-paths.txt"; printf '%s\n' /incan-toolchain.json; } | LC_ALL=C sort > "$RUNNER_TEMP/native-expected-paths.txt"
find "$root" ! -type d -print | sed "s|^$root||" | LC_ALL=C sort > "$RUNNER_TEMP/native-actual-paths.txt"
cmp -s "$RUNNER_TEMP/native-expected-paths.txt" "$RUNNER_TEMP/native-actual-paths.txt" \
  || { echo "::error::native root contains unexpected entries"; exit 1; }
jq -r '.[] | [.path, .sha256, .bytes] | @tsv' "$RUNNER_TEMP/native-headers.json" > "$RUNNER_TEMP/native-installed-checks.tsv"
while IFS=$'\t' read -r source expected bytes; do
  destination="$root$source"
  [ -f "$destination" ] && [ ! -L "$destination" ]
  case "$(realpath -e "$destination")" in "$root"/*) ;; *) echo "::error::installed header escaped sysroot"; exit 1;; esac
  [ "$(sha256sum "$destination" | cut -d ' ' -f1)" = "$expected" ] && [ "$(stat -c '%s' "$destination")" = "$bytes" ] \
    || { echo "::error::installed header bytes differ: $source"; exit 1; }
done < "$RUNNER_TEMP/native-installed-checks.tsv"
jq -n --arg cc "$cc" --arg cxx "$cxx" --arg ar "$ar" --arg sysroot "$root" \
  '{cc: $cc, cxx: $cxx, ar: $ar, sysroot: $sysroot}' > "$RUNNER_TEMP/native-owner-arguments.json"
