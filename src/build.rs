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
        if info.include_paths.len() > 0 {
            let paths = env::join_paths(info.include_paths).unwrap();
            println!("cargo:include={}", paths.to_str().unwrap());
        }
    }

    println!("cargo:rustc-link-lib=lcms2");
}
