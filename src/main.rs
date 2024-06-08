extern crate libc;
use std::ffi::CString;
use std::env;

fn main() {
    unsafe {
        let system = libc::system;
        let clear_string = match env::consts::OS {
            "windows" => "cls",
            _ => "clear"
        };
        let cclr = CString::new(clear_string).unwrap();
        system(cclr.as_ptr() as *const u8);
    }
}
