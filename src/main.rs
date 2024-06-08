extern crate libc;
use std::ffi::CString;

fn main() {
    unsafe {
        let system = libc::system;
        let cclr = CString::new("clear").unwrap();
        system(cclr.as_ptr() as *const u8);
    }
}
