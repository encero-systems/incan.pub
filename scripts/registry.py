#!/usr/bin/env python3
"""Build and validate the incan.pub static index from the committed adoption manifests.

The index is a projection of the manifests, never authored by hand: `build` regenerates it deterministically and
`check` refuses a repository whose manifests, committed generated inputs, or index disagree. Paths follow the
crates.io sparse-index scheme so one client implementation reads both registries.

Usage:
    scripts/registry.py build   # rewrite index/ from crates-io/**/loaf.toml
    scripts/registry.py check   # validate manifests and require index/ to be in sync (exit 1 otherwise)
"""
from __future__ import annotations

import hashlib
import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MANIFEST_GLOB = "crates-io/*/*/loaf.toml"
SHA256 = re.compile(r"^sha256:[0-9a-f]{64}$")
BINDING_KEYS = ("toolchain", "target", "profile", "features")
FACT_KEYS = ("cfg", "out", "link", "tool", "harvested-from")
PROFILES = ("release", "debug")


class Problem(Exception):
    pass


def sparse_index_path(name: str) -> Path:
    """The crates.io sparse-index location for one package name."""
    lower = name.lower()
    if len(lower) == 1:
        return Path("index") / "1" / lower
    if len(lower) == 2:
        return Path("index") / "2" / lower
    if len(lower) == 3:
        return Path("index") / "3" / lower[0] / lower
    return Path("index") / lower[:2] / lower[2:4] / lower


def sha256_file(path: Path) -> str:
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def load_manifest(path: Path) -> dict:
    """Parse one adoption manifest and refuse anything outside the documented format."""
    relative = path.relative_to(ROOT)
    try:
        manifest = tomllib.loads(path.read_text())
    except tomllib.TOMLDecodeError as error:
        raise Problem(f"{relative}: not valid TOML: {error}") from None
    project = manifest.get("project") or {}
    source = manifest.get("source") or {}
    name, version = project.get("name"), project.get("version")
    expected_dir = ROOT / "crates-io" / str(name) / str(version)
    if not name or not version or path.parent != expected_dir:
        raise Problem(f"{relative}: [project] name/version must match its directory")
    if source.get("registry") != "https://github.com/rust-lang/crates.io-index":
        raise Problem(f"{relative}: [source] registry must be the crates.io index")
    if not SHA256.match(str(source.get("checksum", ""))):
        raise Problem(f"{relative}: [source] checksum must be `sha256:` plus 64 lowercase hex digits")
    rust = manifest.get("rust") or {}
    unknown = set(manifest) - {"project", "source", "rust"}
    if unknown:
        raise Problem(f"{relative}: unknown top-level tables {sorted(unknown)}")
    if set(rust) - {"facts"}:
        raise Problem(f"{relative}: [rust] holds only `facts`")
    records = rust.get("facts") or []
    if not records:
        raise Problem(f"{relative}: at least one [[rust.facts]] record is required")
    bindings = []
    for index, record in enumerate(records):
        label = f"{relative}: [[rust.facts]][{index}]"
        unknown = set(record) - set(BINDING_KEYS) - set(FACT_KEYS)
        if unknown:
            raise Problem(f"{label}: unknown keys {sorted(unknown)}; script-emitted environment has no key here")
        for key in BINDING_KEYS:
            if key not in record:
                raise Problem(f"{label}: missing binding key `{key}`")
        if record["profile"] not in PROFILES:
            raise Problem(f"{label}: profile must be one of {PROFILES}")
        features = record["features"]
        if features != sorted(features) or len(set(features)) != len(features):
            raise Problem(f"{label}: features must be sorted and unique")
        if "cfg" not in record:
            raise Problem(f"{label}: `cfg` is required; an empty list is a stated fact")
        cfg = record["cfg"]
        if cfg != sorted(cfg) or len(set(cfg)) != len(cfg):
            raise Problem(f"{label}: cfg must be sorted and unique")
        for entry in record.get("out", []):
            for key in ("name", "path", "digest"):
                if key not in entry:
                    raise Problem(f"{label}: out entry missing `{key}`")
            committed = path.parent / entry["path"]
            if not committed.is_file() or ".." in Path(entry["path"]).parts:
                raise Problem(f"{label}: out `{entry['name']}` names a missing or escaping file {entry['path']}")
            if sha256_file(committed) != entry["digest"]:
                raise Problem(f"{label}: out `{entry['name']}` digest does not match the committed bytes")
        for reserved in ("link", "tool"):
            if reserved in record:
                raise Problem(f"{label}: `{reserved}` is reserved until a publisher-side work grammar exists")
        harvested = record.get("harvested-from")
        if harvested is not None and not SHA256.match(str(harvested)):
            raise Problem(f"{label}: harvested-from must be a `sha256:` receipt identity")
        binding = tuple(record[key] if key != "features" else tuple(features) for key in BINDING_KEYS)
        if binding in bindings:
            raise Problem(f"{label}: duplicate binding {binding}")
        bindings.append(binding)
    return manifest


def index_entry(path: Path, manifest: dict) -> dict:
    """One index line: enough for a client to select the manifest and know which selections it binds."""
    return {
        "name": manifest["project"]["name"],
        "vers": manifest["project"]["version"],
        "cksum": manifest["source"]["checksum"],
        "source": "crates-io",
        "manifest": path.relative_to(ROOT).as_posix(),
        "facts": [
            {key: record[key] for key in BINDING_KEYS}
            for record in manifest["rust"]["facts"]
        ],
    }


def build_index() -> dict[Path, str]:
    """Return the complete index as {path: content}, deterministic in manifest order."""
    entries: dict[str, list[dict]] = {}
    for path in sorted(ROOT.glob(MANIFEST_GLOB)):
        manifest = load_manifest(path)
        entries.setdefault(manifest["project"]["name"], []).append(index_entry(path, manifest))
    files: dict[Path, str] = {}
    for name, lines in entries.items():
        lines.sort(key=lambda entry: tuple(int(part) if part.isdigit() else part for part in re.split(r"[.+-]", entry["vers"])))
        files[sparse_index_path(name)] = "".join(json.dumps(line, separators=(",", ":"), sort_keys=True) + "\n" for line in lines)
    return files


def current_index() -> dict[Path, str]:
    index = ROOT / "index"
    return {path.relative_to(ROOT): path.read_text() for path in sorted(index.rglob("*")) if path.is_file()}


def main(argv: list[str]) -> int:
    command = argv[1] if len(argv) > 1 else "check"
    try:
        wanted = build_index()
    except Problem as error:
        print(f"refused: {error}", file=sys.stderr)
        return 1
    if command == "build":
        for path in list(current_index()):
            (ROOT / path).unlink()
        for path, content in wanted.items():
            (ROOT / path).parent.mkdir(parents=True, exist_ok=True)
            (ROOT / path).write_text(content)
        print(f"index: {len(wanted)} package(s) written")
        return 0
    if command == "check":
        actual = current_index()
        if actual != wanted:
            stale = sorted(set(actual) ^ set(wanted)) or sorted(path for path in wanted if actual.get(path) != wanted[path])
            print(f"refused: index/ is out of date for {[path.as_posix() for path in stale]}; run scripts/registry.py build", file=sys.stderr)
            return 1
        print(f"ok: {len(wanted)} package(s), index in sync")
        return 0
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
