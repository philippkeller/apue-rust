/// Figure 12.11: A nonreentrant version of getenv
///
/// Finding: some fun with pointers and 0 terminated strings :-)

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{c_char, printf};
use std::ffi::CStr;

extern "C" {
    pub static environ: *const *const c_char;
}

const MAXSTRINGSZ:usize = 4096;
static mut BUF:[c_char;MAXSTRINGSZ] = [0;MAXSTRINGSZ];

fn getenv(name: &str) {
    unsafe {
        let mut cmp = name.to_owned();
        cmp.push_str("=");
        let mut i = 0isize;
        loop {
            if *environ.offset(i) == std::ptr::null() {
                break
            }
            let s = CStr::from_ptr(*(environ.offset(i as _))).to_str().expect("no valid string");
            if s.starts_with(&cmp) {
                for (j, c) in s.chars().enumerate() {
                    BUF[j] = c as _;
                }
                BUF[s.len()] = 0;
                break
            }
            i += 1;
        }
    }
}

fn main() {
    getenv("PATH");
    unsafe {
        printf(cstr!("%s\n"), BUF.as_ptr());
    }
}