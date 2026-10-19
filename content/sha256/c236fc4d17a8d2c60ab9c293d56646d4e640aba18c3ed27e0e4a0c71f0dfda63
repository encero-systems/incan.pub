# Changelog

<!-- Ref: https://keepachangelog.com/en/1.1.0/ -->

## [Unreleased]

## [0.0.4] - 2026-09-07

### Added

- Added `ToCharString` for `str`. ([#33](https://github.com/astral-sh/char_str/pull/33))
- Added owned-string collection conversions, `Extend<CharStr>`, and `AsRef<Path>` for both compact string types. ([#34](https://github.com/astral-sh/char_str/pull/34))
- Added const `as_static_str()` methods to recover static string slices. ([#35](https://github.com/astral-sh/char_str/pull/35))

### Changed

- Replaced runtime `memcpy` calls in inline construction with word loads or fixed-size copies. ([#36](https://github.com/astral-sh/char_str/pull/36))

## [0.0.3] - 2026-09-07

### Changed

- Updated the optional `get-size2` dependency to 0.11.0. ([#31](https://github.com/astral-sh/char_str/pull/31))
- Excluded the release script from the published crate. ([#30](https://github.com/astral-sh/char_str/pull/30))

## [0.0.2] - 2026-07-16

### Added

- Added `CharStr::new_inline`, `CharStr::new_heap`, and `CharStr::try_new_heap`
  for explicit control over string storage. ([#28](https://github.com/astral-sh/char_str/pull/28))

## [0.0.1] - 2026-07-15

### Changed

- Forked [`lean_string` 0.6.1](https://github.com/ryota2357/lean_string/releases/tag/v0.6.1)
  as `char_str` for use in Ruff and ty.
- Renamed `LeanString` to `CharString` and `LeanStr` to `CharStr`.
- Added optional `salsa` and `get-size` integrations for ty.

[Unreleased]: https://github.com/astral-sh/char_str/compare/v0.0.4...HEAD
[0.0.4]: https://github.com/astral-sh/char_str/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/astral-sh/char_str/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/astral-sh/char_str/compare/0.0.1...v0.0.2
[0.0.1]: https://github.com/astral-sh/char_str/releases/tag/0.0.1
