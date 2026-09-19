# Harvest proposal

A harvest proposal is what the release bake emits with `--harvest-dir` (`oven legacy-cargo bake-loafs --envelope release --harvest-dir DIR`, one proposal per registry unit whose build script the compatibility publisher observed, per profile) and what `incan-pub add-fact` admits: one bound fact record for one crates.io package version, observed under one exact selection, with the evidence that observation left behind.

Harvest is observation. Admitting the proposal is declaration. The two are kept apart so that what Cargo happened to do on one machine never becomes a registry fact without a reviewable step in between.

## Shape

A JSON file beside any committed generated inputs it names. It mirrors the rendered record — `project`, `source`, one `rust.facts` entry — and adds `evidence`:

```json
{
  "project": { "name": "serde_core", "version": "1.0.228" },
  "source": {
    "registry": "https://github.com/rust-lang/crates.io-index",
    "checksum": "sha256:41d385c7d4ca58e59fc732af25c3983b67ac852c1a25000afe1175de458b67ad"
  },
  "rust": {
    "facts": [
      {
        "toolchain": "rustc 1.98.0 (88d9e12ae 2026-08-18)",
        "target": "aarch64-apple-darwin",
        "profile": "release",
        "features": ["alloc", "default", "result", "std"],
        "cfg": [],
        "out": [{ "name": "private.rs", "path": "out/private.rs", "digest": "sha256:…" }]
      }
    ]
  },
  "evidence": {
    "method": "compatibility publisher observation",
    "receipt": "sha256:…",
    "rustc_identity": "sha256:…",
    "host": "aarch64-apple-darwin",
    "hazards": []
  }
}
```

`evidence.hazards` names ambient variables present when the publisher ran that a stable toolchain never has (`RUSTC_BOOTSTRAP`); the harvest records them, admission refuses a proposal that names any. Other evidence keys are recorded verbatim on the event.

Exactly one `rust.facts` record per proposal. `out` paths are relative to the proposal file; admission copies the named files under `crates-io/<name>/<version>/out/` and refuses a file that would overwrite a committed input with different bytes. The directory a bake writes is `DIR/<name>-<version>-<profile>/proposal.json` beside `out/…`. The proposal is JSON rather than TOML because it is an intermediate the compiler writes and the tool reads, and the event it becomes is JSON; the rendered `loaf.toml` stays the consumer's format.

## What admission checks

- The package version has a `publish` event, or `--publish [--notes FILE]` creates it from the proposal's name, version and checksum, with `--notes` or else a top-level `notes` string in the proposal as the record's header; either way the proposal's checksum equals the record's.
- `evidence.hazards` is empty.
- The binding is not yet present, or is present with identical `cfg` and `out` (the proposal is then a no-op).
- `cfg` and `features` are sorted and unique; every `out` file exists and matches its digest; no key outside the record vocabulary.
- `evidence.receipt`, when present, becomes the record's `harvested-from` and must be a `sha256:` identity.

## What a harvest must not do

A harvest that ran with ambient state a stable publisher never has — `RUSTC_BOOTSTRAP`, a nightly compiler, host tooling leaking into probes — is not a fact of the toolchain. `proc-macro2` emits `proc_macro_span` only under such state; the record for it does not carry that answer. The bake records what it ran under in `evidence.hazards`, and admission refuses a proposal that names any.
