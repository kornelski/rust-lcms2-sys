//! Check out [higher-level wrapper](https://kornelski.github.io/rust-lcms2/lcms2/) for usage in Rust.
//!
//! See [LCMS documentation](https://kornelski.github.io/rust-lcms2-sys/) for more information about these functions.
#![doc(html_logo_url = "https://kornelski.github.io/rust-lcms2/lcms_logo.png")]
#![doc(html_root_url = "https://docs.rs/lcms2-sys")]

pub mod ffi;
pub use crate::ffi::*;
use std::mem::MaybeUninit;

#[doc(hidden)]
extern crate libc;

impl CIEXYZ {
    #[must_use]
    pub fn d50() -> &'static CIEXYZ {
        unsafe { cmsD50_XYZ() }
    }
}

impl CIExyY {
    #[must_use]
    pub fn d50() -> &'static CIExyY {
        unsafe { cmsD50_xyY() }
    }
}

impl From<CIEXYZ> for CIExyY {
    fn from(f: CIEXYZ) -> Self {
        unsafe {
            let mut new = MaybeUninit::uninit();
            cmsXYZ2xyY(new.as_mut_ptr(), &f);
            new.assume_init()
        }
    }
}

impl From<CIExyY> for CIEXYZ {
    fn from(f: CIExyY) -> Self {
        unsafe {
            let mut new = MaybeUninit::uninit();
            cmsxyY2XYZ(new.as_mut_ptr(), &f);
            new.assume_init()
        }
    }
}

impl From<CIELab> for CIELCh {
    fn from(f: CIELab) -> Self {
        unsafe {
            let mut new = MaybeUninit::uninit();
            cmsLab2LCh(new.as_mut_ptr(), &f);
            new.assume_init()
        }
    }
}

impl From<CIELCh> for CIELab {
    fn from(f: CIELCh) -> Self {
        unsafe {
            let mut new = MaybeUninit::uninit();
            cmsLCh2Lab(new.as_mut_ptr(), &f);
            new.assume_init()
        }
    }
}

#[test]
fn it_works() {
    unsafe {
        let c = cmsCreateContext(std::ptr::null_mut(), std::ptr::null_mut());
        cmsDeleteContext(c);

        let xyz: CIEXYZ = ::std::default::Default::default();
        let _: CIExyY = xyz.into();
    }
}
