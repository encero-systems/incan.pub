# External-source record

An external-source record is the registry's publication *about* a package whose source it does not host: a crates.io crate. It binds to that source by checksum and declares, per bound selection, the RFC 119 build facts Oven needs to compile the crate without executing its build script. RFC 125 names this object as one of the two publications a baked asset may derive from.

A record is not a file anyone edits. It is the fold of the events published for `crates-io/<name>@<version>`; `incan-pub build` renders it as `crates-io/<name>/<version>/loaf.toml` for consumers that read `loaf.toml`.

## Events that make up a record

| event | meaning | admission |
|---|---|---|
| `publish` | the record exists for one `(name, version)` and names the crates.io source checksum | published once; checksum is the one crates.io publishes for that version |
| `adopt` | the version is adopted as a Loaf: its Rust facet, features, dependencies, provenance, and the digests of its manifest and source Loaf archive | once per version, an identical repeat is a no-op; the manifest digest is the digest of the manifest the payload renders; every dependency active under the default features is already adopted; on a published record, the checksum matches |
| `fact` | one bound fact record was harvested | binding not yet present, or present with identical `cfg` and `out`; committed `out` bytes match their digests; no key outside the vocabulary below |
| `attest` | an Oven-baked unit for one binding was proven equivalent to the Cargo-built unit | names an existing binding; carries the asset digest, the RFC 124 unit identity and an attestation reference; sets the binding's status to `attested` |
| `asset` | a baked unit for one binding is available | names an existing binding, the archive digest, unit identity, builder kind (`publisher`, `registry`, `local`) and attestation |
| `yank` / `unyank` | governance | recorded; `yanked` on the index line |
| `advisory` | something a consumer of the record should know, in `text` | non-empty `text`; rendered as `# Advisory:` header lines of the record and counted on the index line (`advisories`) |

Every event carries `schema`, `kind`, `at`, `actor`, `subject`, and an `id` that is the digest of its canonical body; a tampered event fails admission. In the interim git transport the commit is the signature and `HEAD` is the checkpoint; the `signature` field is reserved for the signed form.

## The rendered `loaf.toml`

For an adopted version, the rendered `loaf.toml` is the manifest the source Loaf archive carries, in RFC 119's Rust facet grammar (`[project]` with `name = "crates-io/<name>"` and `version`, `[project.features]`, `[rust]`, `[rust.source]` when the library root is not `src/lib.rs`, and `[dependencies]`), followed by the bound facts below. A dependency key declared once per target holds an array of entry tables, each with its own `target`. A version not yet adopted renders the v0 record instead:

### `[project]` — `name`, `version`.

### `[source]` — `registry`, `checksum` (`sha256:` digest of the published `.crate` archive).

### `[[rust.facts]]` — one per bound selection, sorted by binding.

| key | meaning |
|---|---|
| `toolchain` | exact `rustc -vV` identity the answers were derived under |
| `target` | target triple |
| `profile` | `release` or `debug` |
| `features` | complete enabled Cargo feature set, sorted |
| `cfg` | `--cfg` answers, sorted; an empty list is a stated fact |
| `out` | committed generated inputs: `name`, `path` (relative to the manifest), `digest` |
| `link` | publisher-side native compilation: a named producer, its target predicate, the exact `executable` it runs, explicit `environment`, the complete `sources` closure, the `objects` it compiles them into, and the `library` those objects produce |
| `tool` | publisher-side generation: a named producer, its target predicate, the exact `executable`, ordered `arguments`, explicit `environment`, the complete `inputs` closure, and the `outputs` contract |
| `harvested-from` | the compatibility receipt whose capture proposed the record, when the harvest recorded one |

Each entry of `objects` is one compile: `name`, `language` (`c`, `cpp` or `assembly`) and ordered `arguments`. An object names its own file exactly once, as the single output its argv references, so the name and the command cannot drift; its argv references at least one declared source, and every declared source is compiled by some object, which is what makes `sources` a closure rather than a bag. Object and source names share one namespace. No archiver appears in a record: the archive is a deterministic function of the objects, and its digest belongs to the asset's receipt.

`out.name` and `tool.outputs[].path` use the same path namespace relative to `OUT_DIR`; an `out` entry's `path` only says where its committed bytes sit. A generated file or tree is declared exactly once — committed as an `out` input because it was merely observed, or produced by a declared `tool` — and equal or overlapping paths across those fields are refused.

`link` and `tool` are rendered as `[[rust.facts.link]]` and `[[rust.facts.tool]]` sub-tables of the record they belong to. A producer name is claimed once across both lists; an argument or environment entry names a literal or one of that producer's own declared artifacts, never a path; an `outputs` entry carries no digest, because the product bytes are an asset rather than a fact.

A consumer applies a record only when its own selection equals the binding exactly. Script-emitted environment has no key: a value no compilation observes is not a fact; a value one does observe is a typed constant this record does not yet model ([encero-systems/incan#1666](https://github.com/encero-systems/incan/issues/1666)).

### The vocabulary is closed

A record states only the keys in the table above, and `link` and `tool` are reserved: RFC 119 names them as declared build work, and a proposal that carries either is refused by name until their manifest grammar is fixed. A unit that needs them is refused by the harvest rather than declared short.

The vocabulary grows only with the compiler's `RustFactRecord`, because the rendered record is a manifest the compiler parses; a key the compiler does not read is not a fact it can apply. A key entering the vocabulary must satisfy what the existing keys satisfy:

| requirement | why |
|---|---|
| every byte the key names is bound by a `sha256:` digest, under an owner that can produce it: committed beside the record for a generated input, or owner-relative inside an already-checksummed source or an immutable toolchain or provider for a declared input or executable | a consumer compiles from the record without running anything, and a publisher that cannot produce the exact bytes refuses rather than substituting; an unbound byte, an absolute or escaping path, or a URI is refused |
| lists are sorted and duplicate-free; a logical name is claimed once per record | RFC 119 refuses ambiguity rather than resolving it by path or order |
| an omitted key and a stated absence — an empty list or table — are one fact | an explicit "no linked libraries" and no key at all say the same thing, and the record renders both by omitting the key |
| the value is derived from the observation alone | no capture ordering, no absolute path, no host-specific spelling: two harvests of one binding must produce the same bytes |
| a repeated binding repeats the key exactly | immutability compares every fact key except the binding and `harvested-from`, so a key is immutable from the day it is admitted |

A record declares work and its inputs; the product of that work is an `asset`. A declared `link` names its sources and the library it produces, not the built archive; the archive is an attested asset or the consumer's own bake.

## Status

The index line carries, per binding, `status`: `harvested` until an `attest` event exists for it, then `attested`. A record's truth has three sources, and the registry stores references to them rather than judging it: the harvest evidence (what Cargo observed), the attested asset (Oven reproduced the bytes), and the consumer's cross-check while both paths coexist. A consumer may refuse `harvested` records by policy.
