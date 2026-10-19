# char_str

[![Crates.io](https://img.shields.io/crates/v/char_str.svg)](https://crates.io/crates/char_str)
[![Documentation](https://docs.rs/char_str/badge.svg)](https://docs.rs/char_str)

Compact owned strings with mutable and immutable variants.

`char_str` is a fork of [`lean_string`](https://github.com/ryota2357/lean_string) with
customizations for use in [Ruff](https://github.com/astral-sh/ruff) and
[ty](https://docs.astral.sh/ty/).

```rust
use char_str::{CharStr, format_char_str};

let module = "mod";
let name = format_char_str!("pkg.{module}");
assert_eq!(name, "pkg.mod");
assert!(name.len() <= CharStr::INLINE_CAPACITY);
```

## License

This crate is licensed under the MIT license.
