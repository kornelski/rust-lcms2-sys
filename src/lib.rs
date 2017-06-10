//! See [LCMS documentation](https://pornel.github.io/rust-lcms2-sys/) for more information about these functions.

pub mod ffi;
pub use ffi::*;

#[doc(hidden)]
extern crate libc;

#[test]
fn it_works() {
    unsafe {
        let c = cmsCreateContext(std::ptr::null_mut(), std::ptr::null_mut());
        cmsDeleteContext(c);

        let xyz:CIEXYZ = ::std::default::Default::default();
        let _:CIExyY = xyz.into();
    }
}
