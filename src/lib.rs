//! See [LCMS documentation](https://pornel.github.io/rust-lcms2-sys/) for more information about these functions.

pub mod ffi;
pub use ffi::*;
use std::mem;

#[doc(hidden)]
extern crate libc;

impl CIEXYZ {
    pub fn d50() -> &'static CIEXYZ {
        unsafe { cmsD50_XYZ() }
    }
}

impl CIExyY {
    pub fn d50() -> &'static CIExyY {
        unsafe { cmsD50_xyY() }
    }
}

impl From<CIEXYZ> for CIExyY {
    fn from(f: CIEXYZ) -> Self {
        unsafe {
            let mut new = mem::uninitialized();
            cmsXYZ2xyY(&mut new, &f);
            new
        }
    }
}

impl From<CIExyY> for CIEXYZ {
    fn from(f: CIExyY) -> Self {
        unsafe {
            let mut new = mem::uninitialized();
            cmsxyY2XYZ(&mut new, &f);
            new
        }
    }
}

impl From<CIELab> for CIELCh {
    fn from(f: CIELab) -> Self {
        unsafe {
            let mut new = mem::uninitialized();
            cmsLab2LCh(&mut new, &f);
            new
        }
    }
}

impl From<CIELCh> for CIELab {
    fn from(f: CIELCh) -> Self {
        unsafe {
            let mut new = mem::uninitialized();
            cmsLCh2Lab(&mut new, &f);
            new
        }
    }
}

#[test]
fn it_works() {
    unsafe {
        let c = cmsCreateContext(std::ptr::null_mut(), std::ptr::null_mut());
        cmsDeleteContext(c);

        let xyz:CIEXYZ = ::std::default::Default::default();
        let _:CIExyY = xyz.into();
    }
}
