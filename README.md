# [Little CMS](http://www.littlecms.com) bindings for [Rust](https://www.rust-lang.org/)

Makes [LCMS C API](https://github.com/mm2/Little-CMS) available in Rust. There's also a [higher-level wrapper](https://github.com/pornel/rust-lcms2).

It's up to date with version 2.8.

## Differences from [the C API](https://pornel.github.io/rust-lcms2-sys/)

 * Type names don't have the `cms` prefix, e.g. `cmsColorSpace` is `ColorSpace`.
     * All C function names remained the same (with the prefix, e.g. `fn cmsReadTag()`).
 * Enum values don't have the `cmsSig` prefix, e.g. `cmsSigLabData` is `LabData`.
 * Some arguments use more specific type, e.g. `Intent::Perceptual` enum instead of `INTENT_PERCEPTUAL` integer.
