# Adoption manifest format

An adoption manifest is a `loaf.toml` at `crates-io/<name>/<version>/loaf.toml`. It describes one crates.io package version and declares, per bound selection, the build facts Oven needs to compile it without executing its build script.

## `[project]`

| key | meaning |
|---|---|
| `name` | the crates.io package name |
| `version` | the exact published version |

## `[source]`

| key | meaning |
|---|---|
| `registry` | the registry index the package comes from |
| `checksum` | `sha256:` digest of the published `.crate` archive, as `oven.lock` and `Cargo.lock` record it |

A manifest applies only to the source its checksum names. A different checksum for the same name and version is a different package.

## `[[rust.facts]]`

Each record binds one selection and states its facts. A consumer selects the record whose binding equals its own selection exactly; no record matching means the package is not adopted for that selection.

Binding keys:

| key | meaning |
|---|---|
| `toolchain` | the exact `rustc -vV` identity the answers were derived under |
| `target` | the target triple |
| `profile` | `release` or `debug` |
| `features` | the complete enabled Cargo feature set, sorted, including `default` when it is enabled |

Fact keys, each following RFC 119's vocabulary:

| key | meaning |
|---|---|
| `cfg` | the `--cfg` answers the script would have emitted, sorted; an empty list is a stated fact, not an omission |
| `out` | committed generated inputs: `name` (what the source includes by), `path` (relative to the manifest), `digest` (`sha256:` of the committed bytes) |
| `link` | reserved: declared publisher-side link work (RFC 119 `[rust.link]`) |
| `tool` | reserved: declared publisher-side generator work (RFC 119 `[rust.tool]`) |

Provenance keys, optional until harvest records them automatically:

| key | meaning |
|---|---|
| `harvested-from` | the compatibility receipt identity whose capture proposed the record |

## What is deliberately absent

Script-emitted environment (`rustc-env`) has no key. A value no compilation observes is not a fact; a value a compilation does observe is a typed constant the package should declare as such, which is an open question tracked in [encero-systems/incan#1666](https://github.com/encero-systems/incan/issues/1666).
