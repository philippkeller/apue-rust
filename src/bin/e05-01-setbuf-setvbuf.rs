/// Exercise: Implement setbuf using setvbuf

extern crate libc;
extern crate apue;

use apue::*;

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
        let fd = fopen("/etc/passwd".to_ptr(), "r".to_ptr());
        setbuf(fd, std::ptr::null_mut());
    }
}
