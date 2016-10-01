/// Figure 6.11 shows how to use several of the time functions discussed in this chapter.
/// In particular, it shows how strftime can be used to print a string containing
/// the current date and time.

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{tm, time_t, c_char, size_t, printf, exit};
use std::mem::uninitialized;
use apue::CArray;

extern "C" {
    fn time(time: *mut time_t) -> time_t;
    fn localtime(time: *const time_t) -> *mut tm;
    fn strftime(s: *mut c_char,
                maxsize: size_t,
                format: *const c_char,
                timeptr: *const tm)
                -> size_t;
}


fn main() {
    unsafe {
        let buf1: [c_char; 16] = uninitialized();
        let buf2: [c_char; 64] = uninitialized();
        let mut t: time_t = uninitialized();
        time(&mut t);
        let tmp = localtime(&mut t);
        let fmt = "time and date: %r, %a %b %d, %Y";
        if strftime(buf1.as_char(), 16, cstr!(fmt), tmp) == 0 {
            printf(cstr!("buffer length 16 is too small\n"));
        } else {
            printf(cstr!("%s\n"), buf1.as_ptr());
        }
        if strftime(buf2.as_char(), 64, cstr!(fmt), tmp) == 0 {
            printf(cstr!("buffer length 64 is too small\n"));
        } else {
            printf(cstr!("%s\n"), buf2.as_ptr());
        }
        exit(0);
    }
}
