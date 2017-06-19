#[cfg(feature = "dynamic")]
extern crate pkg_config;
#[cfg(any(feature = "static", feature = "static-fallback"))]
extern crate gcc;

use std::env;

fn main() {
    if let Ok(lib_dir) = env::var("LCMS2_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);
    }

    if let Ok(include_dir) = env::var("LCMS2_INCLUDE_DIR") {
        println!("cargo:include={}", include_dir);
    }

    if cfg!(feature = "static") || (!configure_dynamic() && cfg!(feature = "static-fallback")) {
        compile_static();
    }
}

#[cfg(feature = "dynamic")]
fn configure_dynamic() -> bool {
    match pkg_config::find_library("lcms2") {
        Ok(info) => {
            for path in info.include_paths {
                println!("cargo:include={}", path.display());
            }
            true
        },
        Err(err) => {
            println!("cargo:warning=pkg_config failed ({}). Falling back to static build.", err);
            false
        },
    }
}

#[cfg(not(feature = "dynamic"))]
fn configure_dynamic() -> bool {
    false
}

#[cfg(not(any(feature = "static", feature = "static-fallback")))]
fn compile_static() {
    println!("cargo:warning='static' feature of lcms2-sys is disabled, so the library won't be built, and probably won't work at all");
    println!("cargo:rustc-link-lib=lcms2");
}

#[cfg(any(feature = "static", feature = "static-fallback"))]
fn compile_static() {
    gcc::Config::new()
        .include("vendor/include")
        .file("vendor/src/cmsalpha.c")
        .file("vendor/src/cmscam02.c")
        .file("vendor/src/cmscgats.c")
        .file("vendor/src/cmscnvrt.c")
        .file("vendor/src/cmserr.c")
        .file("vendor/src/cmsgamma.c")
        .file("vendor/src/cmsgmt.c")
        .file("vendor/src/cmshalf.c")
        .file("vendor/src/cmsintrp.c")
        .file("vendor/src/cmsio0.c")
        .file("vendor/src/cmsio1.c")
        .file("vendor/src/cmslut.c")
        .file("vendor/src/cmsmd5.c")
        .file("vendor/src/cmsmtrx.c")
        .file("vendor/src/cmsnamed.c")
        .file("vendor/src/cmsopt.c")
        .file("vendor/src/cmspack.c")
        .file("vendor/src/cmspcs.c")
        .file("vendor/src/cmsplugin.c")
        .file("vendor/src/cmsps2.c")
        .file("vendor/src/cmssamp.c")
        .file("vendor/src/cmssm.c")
        .file("vendor/src/cmstypes.c")
        .file("vendor/src/cmsvirt.c")
        .file("vendor/src/cmswtpnt.c")
        .file("vendor/src/cmsxform.c")
        .compile("liblcms2.a");

    println!("cargo:include={}", std::fs::canonicalize("vendor/include").unwrap().display());
}
