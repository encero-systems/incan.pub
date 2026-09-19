# External-source record

An external-source record is the registry's publication *about* a package whose source it does not host: a crates.io crate. It binds to that source by checksum and declares, per bound selection, the RFC 119 build facts Oven needs to compile the crate without executing its build script. RFC 125 names this object as one of the two publications a baked asset may derive from.

A record is not a file anyone edits. It is the fold of the events published for `crates-io/<name>@<version>`; `incan-pub build` renders it as `crates-io/<name>/<version>/loaf.toml` for consumers that read `loaf.toml`.

## Events that make up a record

| event | meaning | admission |
|---|---|---|
| `publish` | the record exists for one `(name, version)` and names the crates.io source checksum | published once; checksum is the one crates.io publishes for that version |
| `fact` | one bound fact record was harvested | binding not yet present, or present with identical `cfg` and `out`; committed `out` bytes match their digests; no key outside the vocabulary below |
| `attest` | an Oven-baked unit for one binding was proven equivalent to the Cargo-built unit | names an existing binding; carries the asset digest, the RFC 124 unit identity and an attestation reference; sets the binding's status to `attested` |
| `asset` | a baked unit for one binding is available | names an existing binding, the archive digest, unit identity, builder kind (`publisher`, `registry`, `local`) and attestation |
| `yank` / `unyank` | governance | recorded; `yanked` on the index line |
| `advisory` | something a consumer of the record should know, in `text` | non-empty `text`; rendered as `# Advisory:` header lines of the record and counted on the index line (`advisories`) |

Every event carries `schema`, `kind`, `at`, `actor`, `subject`, and an `id` that is the digest of its canonical body; a tampered event fails admission. In the interim git transport the commit is the signature and `HEAD` is the checkpoint; the `signature` field is reserved for the signed form.

## The rendered `loaf.toml`

### `[project]` — `name`, `version`.

### `[source]` — `registry`, `checksum` (`sha256:` digest of the published `.crate` archive, as `oven.lock` records it).

### `[[rust.facts]]` — one per bound selection, sorted by binding.

| key | meaning |
|---|---|
| `toolchain` | exact `rustc -vV` identity the answers were derived under |
| `target` | target triple |
| `profile` | `release` or `debug` |
| `features` | complete enabled Cargo feature set, sorted |
| `cfg` | `--cfg` answers, sorted; an empty list is a stated fact |
| `out` | committed generated inputs: `name`, `path` (relative to the manifest), `digest` |
| `harvested-from` | the compatibility receipt whose capture proposed the record, when the harvest recorded one |

A consumer applies a record only when its own selection equals the binding exactly. Script-emitted environment has no key: a value no compilation observes is not a fact; a value one does observe is a typed constant this record does not yet model ([encero-systems/incan#1666](https://github.com/encero-systems/incan/issues/1666)).

## Status

The index line carries, per binding, `status`: `harvested` until an `attest` event exists for it, then `attested`. A record's truth has three sources, and the registry stores references to them rather than judging it: the harvest evidence (what Cargo observed), the attested asset (Oven reproduced the bytes), and the consumer's cross-check while both paths coexist. A consumer may refuse `harvested` records by policy.
