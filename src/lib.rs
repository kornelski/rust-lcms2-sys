mod ffi;
extern crate libc;

#[test]
fn it_works() {
    unsafe {
        let c = ffi::cmsCreateContext(std::ptr::null_mut(), std::ptr::null_mut());
        ffi::cmsDeleteContext(c);
    }
}
