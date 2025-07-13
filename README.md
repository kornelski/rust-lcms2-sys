# [Little CMS](http://www.littlecms.com) bindings for [Rust](https://www.rust-lang.org/)

Makes [LCMS C API](https://github.com/mm2/Little-CMS) available in Rust. There's also a [higher-level wrapper](https://lib.rs/lcms2).

It's up to date with version 2.17, and works with older versions too. Linux, macOS and Windows are supported. Other platforms may work, too.

The crate requires Rust 1.65 or later.

## Differences from [the C API](https://kornelski.github.io/rust-lcms2-sys/)

This crate improves bindgen's bindings to be a bit more Rust-friendly:

 * Type names don't have the `cms` prefix, e.g. `cmsColorSpace` is `ColorSpace`.
     * All C function names remained the same (with the prefix, e.g. `fn cmsReadTag()`).
 * Enum values don't have the `cmsSig` prefix, e.g. `cmsSigLabData` is `LabData`.
 * Some arguments use more specific types, e.g. `Intent::Perceptual` enum instead of `INTENT_PERCEPTUAL` integer.

## Dynamic vs static linking configuration

If `LCMS2_LIB_DIR` environmental variable is set, and the path contains either a static or dynamic library, this libary will be used regardless of other settings.

If `LCMS2_STATIC` environmental variable is set, it will prefer static linking instead.

The package supports ["static" and "dynamic"](https://lib.rs/crates/lcms2-sys/features) Cargo [features](http://doc.crates.io/manifest.html#usage-in-end-products). If "dynamic" is enabled (the default) then it will link to system-wide LCMS2 shared library if `pkg-config` is installed and working correctly. Typically you will also need a `lcms2-dev` or similar package installed on the system.

If `pkg-config` doesn't work (e.g. on Windows), or the "static" feature is enabled, it will build bundled LCMS 2.15 from source instead.

For Rust build scripts using this sys crate as a dependency, Cargo will set `DEP_LCMS2_INCLUDE` env var to [joined paths](https://doc.rust-lang.org/stable/std/env/fn.split_paths.html) of include dirs where `lcms2.h` may be found. This is only relevant if you compile C code using the library directly.

## Contributing

This repo uses git submodules, which can be annoying. Make sure you clone with `git clone --recursive`, or run:

```sh
git submodule update --init
```
