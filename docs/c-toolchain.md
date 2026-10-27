# C toolchain release

The C toolchain release is what RFC 159 compiles every Clove (C-library Loaf) with: one Clang build per build host, made from one pinned LLVM source release, plus the platform files for each target. `bake/c-toolchain/release.json` names the release; a C unit's identity names the release, not the host that compiled it, because the reproduction gate below proves every host compiles the same input to the same bytes.

## `bake/c-toolchain/release.json`

| key | meaning |
|---|---|
| `schema` | `incan.c-toolchain-release/1` |
| `llvm` | the LLVM release every Clang build is made from |
| `llvm_source_sha256` | digest of `llvm-project-<llvm>.src.tar.xz` as LLVM publishes it |
| `clang` | build host triple → that host's Clang build |
| `sysroots` | target triple → that target's sysroot |

Each build and sysroot is named by `repository` (relative to `ghcr.io/encero-systems/incan.pub/`), `tag` and `digest`, the digest of its OCI manifest. Consumers pull by digest; the tag is for people. Every artifact can be pulled without signing in.

## Clang builds: `toolchain/clang:<llvm>-<host>`

Built by `.github/workflows/c-toolchain.yml` on each build host from the pinned source with one configuration:

- projects `clang` and `lld`; backends `X86` and `AArch64`; release build without assertions;
- no optional host libraries (zlib, zstd, libxml2, libedit), tests, examples, benchmarks or docs;
- the C++ standard library linked statically on Linux, so the build runs on any glibc 2.28-or-newer host; deployment target macOS 13.0 on Apple hosts; on Windows, built with MSVC against the static C runtime (`MultiThreaded`);
- distribution components `clang`, `clang-resource-headers`, `lld`, `llvm-ar`, `llvm-ranlib`, `llvm-nm`, `llvm-objcopy`, `llvm-strip`, `llvm-objdump`, `llvm-readobj`.

The artifact is one deterministic `clang-<llvm>-<host>.tar.gz` (sorted entries, zero mtimes and owners) holding `clang-<llvm>-<host>/bin`, `lib` and `include`, with the annotations `dev.incan.c-toolchain.llvm`, `.llvm-source-sha256`, `.host` and `.commit`.

| host | runner |
|---|---|
| `x86_64-unknown-linux-gnu` | `ubuntu-latest` |
| `aarch64-unknown-linux-gnu` | `ubuntu-24.04-arm` |
| `aarch64-apple-darwin` | `macos-latest` |
| `x86_64-pc-windows-msvc` | `windows-latest` |

The workflow runs on dispatch, for every host or for the hosts named in `hosts`. A tag is never overwritten: a host whose tag exists keeps its build, and the run skips it.

## Linux sysroots: `toolchain/c-sysroot:glibc-2.28-<target>-<12 hex>`

Built by `.github/workflows/c-sysroot.yml` from PyPA's `manylinux_2_28` image for the target (AlmaLinux 8, glibc 2.28), pinned by the image digest the manifest records. The sysroot holds the development files of the image's installed `glibc`, `glibc-devel`, `glibc-headers`, `kernel-headers`, `libxcrypt`, `libxcrypt-devel` and `libgcc` packages, as `rpm -ql` lists them, plus GCC's startup objects and libgcc archives (`crtbegin*.o`, `crtend*.o`, `libgcc.a`, `libgcc_eh.a`, `libgcc_s.so`) under `usr/lib/gcc/<triple>/<version>`. Every file lives under `usr/`; `lib64` and `lib` link to `usr/lib64` and `usr/lib`, and every link is relative.

The tag's hex is the start of the archive digest. The artifact holds the deterministic archive, a manifest (`incan.c-sysroot/1`: `target`, `glibc`, `image`, `archive`, `os`, `packages` with name, version and arch) and `files.tsv` (path and digest of every file). Before a sysroot is pushed, a program using threads and libm is linked against it with Clang and lld and run inside the image.

The kernel headers hold names that differ only in case (`xt_MARK.h` and `xt_mark.h`); a sysroot is extracted onto a case-sensitive file system.

## Platform files for Apple and Windows targets

The Apple and Windows SDKs cannot be redistributed, so their units are compiled on managed Builders that hold them. The release pins the target spellings and flags that keep those objects independent of the compiling host:

| target | spelling and flags | without them |
|---|---|---|
| Apple | an explicit deployment target, `--target=arm64-apple-macos13.0` | an Apple host writes its own OS version into the object's build-version command |
| Windows (MSVC) | `-mno-incremental-linker-compatible` (`/Brepro` for `clang-cl`) | the COFF header carries the compile time |

## Reproduction gate

`.github/workflows/c-reproduction.yml` runs on every build host `release.json` names, with that host's Clang build:

1. It compiles one freestanding unit for every target in the table above and both Linux targets, with the pinned spellings and flags (`freestanding.tsv`).
2. On every host that can hold the Linux sysroots (Linux and Apple hosts), it compiles each library of `bake/c-toolchain/corpus.tsv` for every Linux target against the target's sysroot with `-O2 -fPIC`, paths relative to the source root and `-ffile-prefix-map` of the source root, and archives the objects in sorted order with `llvm-ar rcsD` (`archives.tsv`, with every object in `objects.tsv`).
3. On the Apple host, it also compiles the corpus for `arm64-apple-macos13.0` against the runner's Xcode SDK. That build is reported, not compared.

The gate passes only if every host that wrote `freestanding.tsv` or `archives.tsv` wrote the same bytes, and at least two hosts wrote each.

`corpus.tsv` columns: `name`, `version`, `url` and `sha256` of the upstream source release, `sources` (globs relative to the source root), `include` (directories) and `defines`, the last three separated by spaces.

| library | version |
|---|---|
| zstd | 1.5.7 |
| zlib | 1.3.1 |
| bzip2 | 1.0.8 |
