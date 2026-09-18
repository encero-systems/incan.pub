#!/usr/bin/env python3
"""incan.pub, interim transport: an append-only event log, and the projections derived from it.

Every state change is an event under `events/`. The external-source records under `crates-io/` and the sparse
index under `index/` are projections of those events and are never edited by hand: `build` regenerates them and
`check` refuses a repository whose events, committed generated inputs, or projections disagree. In this transport a
commit is the signed event, HEAD is the checkpoint, and this script is admission.

Usage:
    scripts/registry.py check [--offline]        validate events, verify crates.io checksums, require projections in sync
    scripts/registry.py build                    regenerate crates-io/**/loaf.toml and index/ from events
    scripts/registry.py publish NAME VERSION CHECKSUM [--notes FILE]
                                                 create the external-source record for one crates.io package version
    scripts/registry.py add-fact PROPOSAL.toml   admit one harvested bound fact record (see docs/harvest-proposal.md)
    scripts/registry.py attest NAME VERSION --toolchain T --target TR --profile P --features a,b \\
                          --asset SHA --unit-identity SHA --attestation REF
                                                 record an equivalence-attested asset for one bound record
"""
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import re
import shutil
import sys
import tomllib
import urllib.error
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EVENTS = ROOT / "events"
EVENT_SCHEMA = "incan.pub/event/1"
SHA256 = re.compile(r"^sha256:[0-9a-f]{64}$")
BINDING_KEYS = ("toolchain", "target", "profile", "features")
PROFILES = ("release", "debug")
EVENT_KINDS = ("publish", "fact", "attest", "asset", "yank", "unyank", "advisory")
CRATES_IO = "https://github.com/rust-lang/crates.io-index"
CRATES_IO_SPARSE = "https://index.crates.io"


class Refused(Exception):
    """An admission refusal; the message says which rule and which subject."""


# ---------------------------------------------------------------- helpers ----------------------------------------


def digest_bytes(data: bytes) -> str:
    return "sha256:" + hashlib.sha256(data).hexdigest()


def canonical(value) -> bytes:
    return json.dumps(value, separators=(",", ":"), sort_keys=True, ensure_ascii=False).encode()


def sparse_index_path(name: str) -> Path:
    lower = name.lower()
    if len(lower) == 1:
        return Path("index") / "1" / lower
    if len(lower) == 2:
        return Path("index") / "2" / lower
    if len(lower) == 3:
        return Path("index") / "3" / lower[0] / lower
    return Path("index") / lower[:2] / lower[2:4] / lower


def crates_io_sparse_url(name: str) -> str:
    return f"{CRATES_IO_SPARSE}/{sparse_index_path(name).relative_to('index').as_posix()}"


def binding_of(record: dict) -> tuple:
    return (record["toolchain"], record["target"], record["profile"], tuple(record["features"]))


def sorted_unique(values: list) -> bool:
    return all(values[i] < values[i + 1] for i in range(len(values) - 1))


# ---------------------------------------------------------------- events -----------------------------------------


def event_body(event: dict) -> dict:
    return {key: value for key, value in event.items() if key not in ("id", "signature")}


def event_id(event: dict) -> str:
    return digest_bytes(canonical(event_body(event)))


def load_events() -> list[dict]:
    """Read the log in sequence order, refusing gaps, tampering, and unknown shapes."""
    events = []
    if not EVENTS.is_dir():
        return events
    files = sorted(EVENTS.glob("*.json"))
    for expected, path in enumerate(files, start=1):
        match = re.match(r"^(\d{6})-([a-z]+)-(.+)\.json$", path.name)
        if not match or int(match.group(1)) != expected:
            raise Refused(f"events/{path.name}: sequence must be contiguous six-digit numbers in file order")
        try:
            event = json.loads(path.read_text())
        except json.JSONDecodeError as error:
            raise Refused(f"events/{path.name}: not valid JSON: {error}") from None
        if event.get("schema") != EVENT_SCHEMA:
            raise Refused(f"events/{path.name}: schema must be {EVENT_SCHEMA}")
        if event.get("kind") not in EVENT_KINDS or event["kind"] != match.group(2):
            raise Refused(f"events/{path.name}: kind must be one of {EVENT_KINDS} and match the file name")
        if event.get("id") != event_id(event):
            raise Refused(f"events/{path.name}: id does not match the canonical event body")
        subject = event.get("subject") or {}
        if subject.get("scope") != "crates-io" or not subject.get("name") or not subject.get("version"):
            raise Refused(f"events/{path.name}: subject must name a crates-io package version")
        if not isinstance(event.get("at"), str) or not isinstance(event.get("actor"), str) or not event["actor"]:
            raise Refused(f"events/{path.name}: `at` and `actor` are required")
        event["_path"] = path
        events.append(event)
    return events


def append_event(kind: str, subject: dict, actor: str, payload: dict) -> Path:
    EVENTS.mkdir(exist_ok=True)
    sequence = len(list(EVENTS.glob("*.json"))) + 1
    event = {
        "schema": EVENT_SCHEMA,
        "kind": kind,
        "at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "actor": actor,
        "subject": subject,
        **payload,
    }
    event["id"] = event_id(event)
    event["signature"] = None  # In this transport the commit carries the signature; kept for the signed form.
    slug = f"{subject['scope']}-{subject['name']}-{subject['version']}".replace("/", "-")
    path = EVENTS / f"{sequence:06d}-{kind}-{slug}.json"
    path.write_text(json.dumps(event, indent=2, sort_keys=True) + "\n")
    return path


# ---------------------------------------------------------------- replay -----------------------------------------


def replay(events: list[dict]) -> dict[tuple[str, str], dict]:
    """Fold the log into the current state, applying every admission rule in order."""
    state: dict[tuple[str, str], dict] = {}
    for event in events:
        label = f"events/{event['_path'].name}"
        subject = event["subject"]
        key = (subject["name"], subject["version"])
        kind = event["kind"]
        if kind == "publish":
            if key in state:
                raise Refused(f"{label}: {key[0]} {key[1]} is already published; versions are immutable")
            source = event.get("source") or {}
            if source.get("registry") != CRATES_IO or not SHA256.match(str(source.get("checksum", ""))):
                raise Refused(f"{label}: source must name the crates.io index and a `sha256:` checksum")
            state[key] = {
                "name": key[0],
                "version": key[1],
                "source": source,
                "notes": event.get("notes", ""),
                "adopter": event["actor"],
                "facts": {},
            }
            continue
        record = state.get(key)
        if record is None:
            raise Refused(f"{label}: {key[0]} {key[1]} has no publish event before this {kind}")
        if kind == "fact":
            fact = validate_fact(event.get("fact") or {}, key, label)
            binding = binding_of(fact)
            existing = record["facts"].get(binding)
            if existing is not None:
                comparable = {k: v for k, v in fact.items() if k in ("cfg", "out")}
                if comparable != {k: v for k, v in existing.items() if k in ("cfg", "out")}:
                    raise Refused(f"{label}: binding {binding} already has a different record; a fact is immutable")
                continue
            fact["status"] = "harvested"
            fact["attestations"] = []
            record["facts"][binding] = fact
        elif kind == "attest":
            binding = binding_of(validate_binding(event.get("binding") or {}, label))
            fact = record["facts"].get(binding)
            if fact is None:
                raise Refused(f"{label}: attestation names a binding with no fact record")
            attestation = event.get("attestation") or {}
            for field in ("asset", "unit_identity", "reference"):
                if not attestation.get(field):
                    raise Refused(f"{label}: attestation requires `{field}`")
            if not SHA256.match(attestation["asset"]) or not SHA256.match(attestation["unit_identity"]):
                raise Refused(f"{label}: attestation asset and unit identity must be `sha256:` identities")
            fact["attestations"].append(attestation)
            fact["status"] = "attested"
        elif kind == "asset":
            asset = event.get("asset") or {}
            for field in ("archive", "unit_identity", "builder", "attestation"):
                if not asset.get(field):
                    raise Refused(f"{label}: asset requires `{field}`")
            if asset["builder"] not in ("publisher", "registry", "local"):
                raise Refused(f"{label}: asset builder must be publisher, registry or local")
            binding = binding_of(validate_binding(event.get("binding") or {}, label))
            if binding not in record["facts"]:
                raise Refused(f"{label}: asset names a binding with no fact record")
            record.setdefault("assets", []).append({"binding": list(binding[:3]) + [list(binding[3])], **asset})
        elif kind in ("yank", "unyank"):
            record["yanked"] = kind == "yank"
        elif kind == "advisory":
            record.setdefault("advisories", []).append(event.get("advisory") or {})
    return state


def validate_binding(binding: dict, label: str) -> dict:
    for key in BINDING_KEYS:
        if key not in binding:
            raise Refused(f"{label}: missing binding key `{key}`")
    if not isinstance(binding["toolchain"], str) or not binding["toolchain"].strip():
        raise Refused(f"{label}: toolchain must name the exact compiler")
    if not isinstance(binding["target"], str) or not binding["target"].strip():
        raise Refused(f"{label}: target must name a triple")
    if binding["profile"] not in PROFILES:
        raise Refused(f"{label}: profile must be one of {PROFILES}")
    features = binding["features"]
    if not isinstance(features, list) or not sorted_unique(features):
        raise Refused(f"{label}: features must be a sorted, unique list")
    return binding


def validate_fact(fact: dict, key: tuple[str, str], label: str) -> dict:
    validate_binding(fact, label)
    unknown = set(fact) - set(BINDING_KEYS) - {"cfg", "out", "harvested-from"}
    if unknown:
        raise Refused(f"{label}: unknown fact keys {sorted(unknown)}; script-emitted environment has no key here")
    if "cfg" not in fact or not isinstance(fact["cfg"], list) or not sorted_unique(fact["cfg"]):
        raise Refused(f"{label}: cfg must be a sorted, unique list; an empty list is a stated fact")
    out_dir = ROOT / "crates-io" / key[0] / key[1]
    names = set()
    for entry in fact.get("out", []):
        for field in ("name", "path", "digest"):
            if field not in entry:
                raise Refused(f"{label}: out entry missing `{field}`")
        relative = Path(entry["path"])
        if relative.is_absolute() or ".." in relative.parts:
            raise Refused(f"{label}: out `{entry['name']}` must be a plain relative path")
        committed = out_dir / relative
        if not committed.is_file():
            raise Refused(f"{label}: out `{entry['name']}` names a missing committed file {entry['path']}")
        if digest_bytes(committed.read_bytes()) != entry["digest"]:
            raise Refused(f"{label}: out `{entry['name']}` digest does not match the committed bytes")
        if entry["name"] in names:
            raise Refused(f"{label}: out `{entry['name']}` is declared twice")
        names.add(entry["name"])
    harvested = fact.get("harvested-from")
    if harvested is not None and not SHA256.match(str(harvested)):
        raise Refused(f"{label}: harvested-from must be a `sha256:` receipt identity")
    return fact


# ---------------------------------------------------------------- projections ------------------------------------


def toml_string(value: str) -> str:
    return json.dumps(value, ensure_ascii=False)


def toml_list(values: list[str]) -> str:
    return "[" + ", ".join(toml_string(value) for value in values) + "]"


def render_manifest(record: dict) -> str:
    """The external-source record as the `loaf.toml` a consumer reads."""
    lines = []
    if record["notes"]:
        lines.extend(f"# {line}".rstrip() for line in record["notes"].strip().splitlines())
        lines.append("#")
    lines.append("# Projection of the incan.pub event log; regenerate with scripts/registry.py build, never edit.")
    lines.append("")
    lines.append("[project]")
    lines.append(f"name = {toml_string(record['name'])}")
    lines.append(f"version = {toml_string(record['version'])}")
    lines.append("")
    lines.append("[source]")
    lines.append(f"registry = {toml_string(record['source']['registry'])}")
    lines.append(f"checksum = {toml_string(record['source']['checksum'])}")
    for binding in sorted(record["facts"]):
        fact = record["facts"][binding]
        lines.append("")
        lines.append("[[rust.facts]]")
        lines.append(f"toolchain = {toml_string(fact['toolchain'])}")
        lines.append(f"target = {toml_string(fact['target'])}")
        lines.append(f"profile = {toml_string(fact['profile'])}")
        lines.append(f"features = {toml_list(fact['features'])}")
        lines.append(f"cfg = {toml_list(fact['cfg'])}")
        if fact.get("out"):
            entries = ", ".join(
                "{ name = %s, path = %s, digest = %s }"
                % (toml_string(o["name"]), toml_string(o["path"]), toml_string(o["digest"]))
                for o in fact["out"]
            )
            lines.append(f"out = [{entries}]")
        if fact.get("harvested-from"):
            lines.append(f"harvested-from = {toml_string(fact['harvested-from'])}")
    return "\n".join(lines) + "\n"


def index_line(record: dict) -> dict:
    """One index line: enough to select the manifest and decide which bindings are adopted, and how firmly."""
    return {
        "name": record["name"],
        "vers": record["version"],
        "cksum": record["source"]["checksum"],
        "source": "crates-io",
        "adopter": record["adopter"],
        "manifest": f"crates-io/{record['name']}/{record['version']}/loaf.toml",
        "yanked": bool(record.get("yanked")),
        "facts": [
            {
                "toolchain": fact["toolchain"],
                "target": fact["target"],
                "profile": fact["profile"],
                "features": fact["features"],
                "status": fact["status"],
            }
            for _, fact in sorted(record["facts"].items())
        ],
        "assets": len(record.get("assets", [])),
    }


def version_key(version: str) -> tuple:
    return tuple(int(part) if part.isdigit() else part for part in re.split(r"[.+-]", version))


def render_projections(state: dict) -> dict[Path, str]:
    files: dict[Path, str] = {}
    by_name: dict[str, list[dict]] = {}
    for (name, _), record in sorted(state.items()):
        files[Path("crates-io") / name / record["version"] / "loaf.toml"] = render_manifest(record)
        by_name.setdefault(name, []).append(index_line(record))
    for name, lines in by_name.items():
        lines.sort(key=lambda line: version_key(line["vers"]))
        files[sparse_index_path(name)] = "".join(
            json.dumps(line, separators=(",", ":"), sort_keys=True) + "\n" for line in lines
        )
    return files


def committed_projections() -> dict[Path, str]:
    files = {}
    for base in ("index", "crates-io"):
        root = ROOT / base
        if not root.is_dir():
            continue
        for path in sorted(root.rglob("*")):
            if path.is_file() and (base == "index" or path.name == "loaf.toml"):
                files[path.relative_to(ROOT)] = path.read_text()
    return files


# ---------------------------------------------------------------- crates.io --------------------------------------


def verify_checksums_online(state: dict) -> list[str]:
    """Confirm each record's checksum is the one crates.io publishes for that version; never fetch an archive."""
    problems = []
    cache: dict[str, dict[str, str]] = {}
    for (name, version), record in sorted(state.items()):
        if name not in cache:
            url = crates_io_sparse_url(name)
            try:
                with urllib.request.urlopen(url, timeout=20) as response:
                    lines = response.read().decode().splitlines()
            except (urllib.error.URLError, TimeoutError) as error:
                raise Refused(f"could not read the crates.io index for `{name}` ({url}): {error}") from None
            cache[name] = {}
            for line in lines:
                if line.strip():
                    entry = json.loads(line)
                    cache[name][entry["vers"]] = entry["cksum"]
        published = cache[name].get(version)
        expected = record["source"]["checksum"].removeprefix("sha256:")
        if published is None:
            problems.append(f"{name} {version}: crates.io publishes no such version")
        elif published != expected:
            problems.append(f"{name} {version}: crates.io checksum {published} differs from the record's {expected}")
    return problems


# ---------------------------------------------------------------- commands ---------------------------------------


def cmd_build(_: argparse.Namespace) -> int:
    state = replay(load_events())
    wanted = render_projections(state)
    for path in committed_projections():
        if path not in wanted:
            (ROOT / path).unlink()
    for path, content in wanted.items():
        (ROOT / path).parent.mkdir(parents=True, exist_ok=True)
        (ROOT / path).write_text(content)
    print(f"built: {len(state)} record(s), {sum(len(r['facts']) for r in state.values())} bound fact(s)")
    return 0


def cmd_check(args: argparse.Namespace) -> int:
    events = load_events()
    state = replay(events)
    wanted = render_projections(state)
    actual = committed_projections()
    if actual != wanted:
        stale = sorted(path.as_posix() for path in set(actual) ^ set(wanted))
        stale += sorted(path.as_posix() for path in wanted if path in actual and actual[path] != wanted[path])
        raise Refused(f"projections are out of date for {stale}; run scripts/registry.py build")
    if not args.offline:
        problems = verify_checksums_online(state)
        if problems:
            raise Refused("crates.io disagrees with the log: " + "; ".join(problems))
    facts = sum(len(record["facts"]) for record in state.values())
    attested = sum(1 for record in state.values() for fact in record["facts"].values() if fact["status"] == "attested")
    print(
        f"ok: {len(events)} event(s), {len(state)} record(s), {facts} bound fact(s) ({attested} attested), "
        f"projections in sync{' (checksums not verified: offline)' if args.offline else ', crates.io checksums verified'}"
    )
    return 0


def cmd_publish(args: argparse.Namespace) -> int:
    if not SHA256.match(args.checksum):
        raise Refused("checksum must be `sha256:` plus 64 lowercase hex digits")
    state = replay(load_events())
    if (args.name, args.version) in state:
        raise Refused(f"{args.name} {args.version} is already published; versions are immutable")
    notes = Path(args.notes).read_text().strip() if args.notes else ""
    path = append_event(
        "publish",
        {"scope": "crates-io", "name": args.name, "version": args.version},
        args.actor,
        {"source": {"registry": CRATES_IO, "checksum": args.checksum}, "notes": notes},
    )
    print(f"published record: {path.relative_to(ROOT)}")
    return cmd_build(args)


def cmd_add_fact(args: argparse.Namespace) -> int:
    proposal_path = Path(args.proposal).resolve()
    proposal = tomllib.loads(proposal_path.read_text())
    project = proposal.get("project") or {}
    source = proposal.get("source") or {}
    facts = (proposal.get("rust") or {}).get("facts") or []
    evidence = proposal.get("evidence") or {}
    name, version = project.get("name"), project.get("version")
    if not name or not version or len(facts) != 1:
        raise Refused("a proposal names one package version and carries exactly one [[rust.facts]] record")
    state = replay(load_events())
    record = state.get((name, version))
    if record is None:
        raise Refused(f"{name} {version} has no external-source record; publish it first")
    if source.get("checksum") != record["source"]["checksum"]:
        raise Refused("the proposal binds a different source checksum than the published record")
    fact = dict(facts[0])
    if evidence.get("receipt"):
        fact["harvested-from"] = evidence["receipt"]
    out_dir = ROOT / "crates-io" / name / version
    for entry in fact.get("out", []):
        staged = proposal_path.parent / entry["path"]
        if not staged.is_file():
            raise Refused(f"proposal out `{entry.get('name')}` names a missing file {entry['path']}")
        destination = out_dir / entry["path"]
        destination.parent.mkdir(parents=True, exist_ok=True)
        if destination.exists() and destination.read_bytes() != staged.read_bytes():
            raise Refused(f"committed generated input {entry['path']} already exists with different bytes")
        shutil.copyfile(staged, destination)
    validate_fact(fact, (name, version), f"proposal {proposal_path.name}")
    existing = record["facts"].get(binding_of(fact))
    if existing is not None:
        comparable = lambda f: {k: v for k, v in f.items() if k in ("cfg", "out")}  # noqa: E731
        if comparable(existing) != comparable(fact):
            raise Refused("the proposal's binding already has a different record; a fact is immutable")
        print(f"already admitted: {name} {version} {binding_of(fact)}")
        return 0
    path = append_event(
        "fact",
        {"scope": "crates-io", "name": name, "version": version},
        args.actor,
        {"fact": fact, "evidence": {k: v for k, v in evidence.items() if k != "receipt"} | ({"receipt": evidence["receipt"]} if evidence.get("receipt") else {})},
    )
    print(f"admitted fact: {path.relative_to(ROOT)}")
    return cmd_build(args)


def cmd_attest(args: argparse.Namespace) -> int:
    features = [feature for feature in args.features.split(",") if feature] if args.features else []
    binding = {"toolchain": args.toolchain, "target": args.target, "profile": args.profile, "features": features}
    state = replay(load_events())
    record = state.get((args.name, args.version))
    if record is None or binding_of(binding) not in record["facts"]:
        raise Refused("attestation must name a published record and one of its bound facts")
    path = append_event(
        "attest",
        {"scope": "crates-io", "name": args.name, "version": args.version},
        args.actor,
        {
            "binding": binding,
            "attestation": {
                "asset": args.asset,
                "unit_identity": args.unit_identity,
                "reference": args.attestation,
            },
        },
    )
    print(f"attested: {path.relative_to(ROOT)}")
    return cmd_build(args)


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(prog="registry.py", description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--actor", default="encero-systems/incan.pub maintainers", help="identity recorded on new events")
    sub = parser.add_subparsers(dest="command", required=True)
    check = sub.add_parser("check")
    check.add_argument("--offline", action="store_true", help="skip crates.io checksum verification")
    check.set_defaults(run=cmd_check)
    sub.add_parser("build").set_defaults(run=cmd_build)
    publish = sub.add_parser("publish")
    publish.add_argument("name")
    publish.add_argument("version")
    publish.add_argument("checksum")
    publish.add_argument("--notes", help="file whose text explains the record; rendered as the manifest header")
    publish.set_defaults(run=cmd_publish)
    add_fact = sub.add_parser("add-fact")
    add_fact.add_argument("proposal")
    add_fact.set_defaults(run=cmd_add_fact)
    attest = sub.add_parser("attest")
    for flag in ("name", "version"):
        attest.add_argument(flag)
    for flag in ("--toolchain", "--target", "--profile", "--asset", "--unit-identity", "--attestation"):
        attest.add_argument(flag, required=True)
    attest.add_argument("--features", default="")
    attest.set_defaults(run=cmd_attest)
    args = parser.parse_args(argv[1:])
    try:
        return args.run(args)
    except Refused as error:
        print(f"refused: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
