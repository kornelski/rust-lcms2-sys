extern crate pkg_config;

use std::env;

fn main() {
    if let Ok(lib_dir) = env::var("LCMS2_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);
    }

    if let Ok(include_dir) = env::var("LCMS2_INCLUDE_DIR") {
        println!("cargo:include={}", include_dir);
    }

    if let Ok(info) = pkg_config::find_library("lcms2") {
        for path in info.include_paths {
            println!("cargo:include={}", path.display());
        }
    } else {
        println!("cargo:rustc-link-lib=lcms2");
    }
}
