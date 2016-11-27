/// Exercise 5.1: Implement setbuf using setvbuf
///
/// $ e01-setbuf-setvbuf

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{c_char, FILE, _IOFBF, _IONBF, setvbuf, fopen, strlen};

unsafe fn setbuf(stream: *mut libc::FILE, buf: *mut c_char) {
    if buf.is_null() {
        // turn off buffering
        setvbuf(stream as *mut FILE, buf, _IONBF, 0);
    } else {
        // set full buffering
        setvbuf(stream as *mut FILE, buf, _IOFBF, strlen(buf));
    }
}

fn main() {
    unsafe {
        let stream = fopen(cstr!("/etc/passwd"), cstr!("r"));
        setbuf(stream, std::ptr::null_mut());
    }
}