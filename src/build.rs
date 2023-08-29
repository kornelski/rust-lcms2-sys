use std::env;
use std::env::consts;
use std::path::Path;

fn main() {
    if let Ok(include_dir) = env::var("LCMS2_INCLUDE_DIR") {
        println!("cargo:include={}", include_dir);
    }

    if let Some(lib_dir) = env::var_os("LCMS2_LIB_DIR") {
        let lib_dir = Path::new(&lib_dir);
        let dylib_name = format!("{}lcms2{}", consts::DLL_PREFIX, consts::DLL_SUFFIX);
        if [&dylib_name, "liblcms2.a", "lcms2.so", "lcms2.lib"].iter().any(|file| lib_dir.join(file).exists()) {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
            println!("cargo:rustc-link-lib=lcms2");
            return;
        }
        println!("cargo:warning=LCMS2_LIB_DIR path ({}) did not contain {dylib_name}", lib_dir.display());
    }

    let requires_static_only = cfg!(feature = "static") || env::var("LCMS2_STATIC").is_ok();
    if requires_static_only || (!configure_pkg_config() && cfg!(feature = "static-fallback")) {
        compile_static();
    }
}

#[cfg(feature = "dynamic")]
fn configure_pkg_config() -> bool {
    match pkg_config::probe_library("lcms2") {
        Ok(info) => {
            let joined_paths = std::env::join_paths(info.include_paths).ok().and_then(|p| p.into_string().ok());
            if let Some(joined_paths) = joined_paths {
                println!("cargo:include={joined_paths}");
                true
            } else {
                println!("cargo:warning=got invalid include paths from pkg-config of lcms2");
                false
            }
        },
        Err(err) => {
            println!("cargo:warning=pkg_config failed ({}). Falling back to static build.", err);
            false
        },
    }
}

#[cfg(not(feature = "dynamic"))]
fn configure_pkg_config() -> bool {
    false
}

#[cfg(not(any(feature = "static", feature = "static-fallback")))]
fn compile_static() {
    println!("cargo:warning='static' feature of lcms2-sys is disabled, so the library won't be built, and probably won't work at all");
    println!("cargo:rustc-link-lib=lcms2");
}

#[cfg(any(feature = "static", feature = "static-fallback"))]
fn compile_static() {
    let mut cc = cc::Build::new();
        cc.include("vendor/include")
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
        .file("vendor/src/cmsxform.c");

    if env::var_os("CARGO_CFG_TARGET_ENDIAN").as_deref() == Some("big".as_ref()) {
        cc.define("CMS_USE_BIG_ENDIAN", Some("1"));
    }

    if cfg!(feature = "lcms2-strict-cgats") {
        cc.define("CMS_STRICT_CGATS", Some("1"));
    }

    if env::var_os("DEBUG").as_deref() != Some("true".as_ref()) {
        cc.define("NDEBUG", Some("1"));
    }

    cc.compile("liblcms2.a");
    println!("cargo:include={}", dunce::canonicalize("vendor/include").unwrap().display());
}
