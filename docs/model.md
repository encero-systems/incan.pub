# The model: what Incan does to publish, what the registry does to manage

`incan.pub` holds three kinds of publication and the events that change them. This page states the loop from both ends, so that the toolchain and the registry are built against the same shape. It follows [RFC 125](https://github.com/encero-systems/incan/blob/main/workspaces/docs-site/docs/RFCs/125_incan_pub_loaf_registry_and_baked_asset_distribution.md); where this interim transport differs from the RFC's signed HTTPS form, the difference is named.

## The things

| thing | what it is | signed by |
|---|---|---|
| **Source Loaf** | archive of a project's `loaf.toml` and the sources its facets select; identity is the archive digest | the publisher |
| **External-source record** | a declaration *about* a crates.io source, bound by checksum: RFC 119 build facts as records bound to one toolchain, target, profile and feature set | the adopter — today, this registry's maintainers |
| **Baked asset** | one RFC 124 unit with its receipt and an attestation binding asset digest → source publication digest → toolchain; builder kind `publisher`, `registry` or `local` | the builder |
| **Index line, asset manifest, rendered `loaf.toml`** | projections of the events and artifacts; never authored | the registry, at a checkpoint |
| **Event** | `publish`, `fact`, `attest`, `asset`, `yank`, `unyank`, `advisory` — append-only, each with a content identity | an identity authorised for the scope |

The registry adds one event kind to RFC 125: **`fact`**. Bound records accrue over time exactly as assets do — a new toolchain, another target — so the record is the immutable unit and the rendered `loaf.toml` is a projection, like the index.

## What Incan does to get things in

1. **`oven publish`** bakes, signs and uploads a source Loaf; publisher-built assets ride along. (RFC 125, unchanged.)
2. **`oven harvest <crate>@<version>`** runs the compatibility publisher once for an exact selection and emits a [harvest proposal](harvest-proposal.md): the bound record and the evidence of its observation. Harvest is observation, not publication.
3. **Admitting the proposal** (`incan-pub add-fact`) publishes it as a `fact` event on `crates-io/<name>@<version>`. This is declaration, by an identity authorised for the `crates-io` scope.
4. **`oven bake` of an adopted crate** compiles it natively from the record. The resulting RFC 124 unit becomes an `asset` event; when its unit identity equals the Cargo-built unit's, an `attest` event records the equivalence and the binding's status becomes `attested`.
5. **Pinning.** The toolchain manifest pins a checkpoint; in this transport, a commit. `oven.lock` records for every unit where it came from and whether an asset or a source bake satisfied it.

## What the registry does to manage them

- **Admit.** Authorisation for the scope; `(name, version)` published once; each event's identity matches its body and the log has no gaps; for a source Loaf, the manifest describes the archive; for an external-source record, the checksum is the one crates.io publishes for that version (the registry reads the crates.io sparse index; it never mirrors archives); records are closed, sorted and bound once; committed generated inputs match their digests. `incan-pub check` is this step.
- **Project.** Rebuild index lines and rendered records from the events and artifacts. `incan-pub build` is this step; nothing is written twice.
- **Attest and checkpoint.** In the signed form, sign index files and publish a periodic checkpoint so mirrors carry no trust. In this transport, the commit is the signature and `HEAD` the checkpoint.
- **Govern.** Yank, advisory and supersession as events; ownership and trusted publishers per scope. The `crates-io` scope is reserved and operated by the toolchain, like the unscoped standard library: a wrong record is wrong bytes for everyone, so only the toolchain's trusted publisher adds facts.
- **Answer, statically.** Which bindings a record covers, and how firmly, is on the index line; which assets are admissible for a plan is in the asset manifest. A client decides before it downloads.

## The boundary

The registry is a notary, not a judge. It cannot know that a `cfg` answer is true. Truth has three sources, and the registry stores references to them: the harvest evidence (what Cargo observed), the attested asset (Oven reproduced the bytes), and the consumer's cross-check of declared against observed facts while both paths coexist. A consumer may refuse `harvested` bindings by policy and accept only `attested` ones. That is where "Oven is real" becomes a property one can check per crate.

## Why the registry is built first

A compiler built against a registry that does not yet exist ends up encoding the interim shape. Everything the compiler needs from here is a projection — an index line, a rendered `loaf.toml` — and the compiler reads only projections. The events, admission and status live here, so they can change without a compiler change.
