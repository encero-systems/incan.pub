## 2.0.0

- Add `no_std` support using `core` and `alloc` only, and raise the MSRV from Rust 1.34.2 to 1.36.0 (https://github.com/image-rs/color_quant/pull/24)
- Add `NeuQuant::color_map_alpha` for retrieving the palette alpha channel, useful for PNG `tRNS` chunks (https://github.com/image-rs/color_quant/pull/23)
- Fix `search_netindex` so palette index 0 is checked and both search directions are handled correctly (https://github.com/image-rs/color_quant/pull/16)
- Fix a sampling prime typo by changing `478` to `487` (https://github.com/image-rs/color_quant/pull/14)
- Forbid unsafe code in the crate (https://github.com/image-rs/color_quant/pull/22)
- Improve and specialize internal math operations used by the quantizer, and document their provenance (https://github.com/image-rs/color_quant/pull/24)

## 1.1.0

- Unify with `image::math::nq` as per https://github.com/image-rs/image/issues/1338 (https://github.com/image-rs/color_quant/pull/10)
  - A new method `lookup` from `image::math::nq` is added
  - More references in docs
  - Some style improvements and better names for functions borrowed from  `image::math::nq`
- Replace the internal `clamp!` macro with the `clamp` function (https://github.com/image-rs/color_quant/pull/8)
