# The registry's published policy

RFC 125 leaves two things to a registry's published policy: which scopes beyond its own it reserves, and which toolchains it supports. [`src/policy.incn`](../src/policy.incn) holds both, and `catalog/policy.json` on the `index` branch publishes them, so a reader gets the policy from the same checkpoint as everything else.

## Reserved scopes

A scope is the first part of a package name. A reserved scope can't be claimed by a community publisher, and a publish to one is refused from any identity not authorized for it.

| scope | what it holds |
|---|---|
| `c` | C libraries adopted from an upstream release |
| `crates-io` | packages adopted from crates.io |
| `encero` | reserved for the project |
| `encero-systems` | reserved for the project |
| `incan` | native packages the project provides |
| `oven` | reserved for the toolchain |
| `std` | reserved for the toolchain |

## Supported toolchains

An asset is kept while its toolchain is supported, and its record names that toolchain as `incan@<commit>`. Once a toolchain leaves this list, the bytes of its assets may be removed by a signed `retire` event; a removed asset is an ordinary miss, and the consumer bakes from source. Nothing is removed because of age or total size.

The supported set is the release the project's CI builds with: `incan@0cb030e9c6d9817c620e481854e7d6f6440fd9d6`, the 0.6.0-dev.8 bundles.
