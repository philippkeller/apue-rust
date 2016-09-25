/// Figure 6.11 shows how to use several of the time functions discussed in this chapter.
/// In particular, it shows how strftime can be used to print a string containing
/// the current date and time.

extern crate libc;
extern crate apue;

use libc::{tm, time_t, c_char, size_t, printf, exit};
use std::mem::uninitialized;
use apue::*;

extern "C" {
    fn time(time: *mut time_t) -> time_t;
    fn localtime(time: *const time_t) -> *mut tm;
    fn strftime(s: *mut c_char,
                maxsize: size_t,
                format: *const c_char,
                timeptr: *const tm)
                -> size_t;
}

trait CArray {
    fn as_char(&self) -> *mut c_char;
}

impl CArray for [c_char] {
    fn as_char(&self) -> *mut c_char {
        self.as_ptr() as *mut c_char
    }
}


fn main() {
    unsafe {
        let buf1: [c_char; 16] = uninitialized();
        let buf2: [c_char; 64] = uninitialized();
        let mut t: time_t = uninitialized();
        time(&mut t);
        let tmp = localtime(&mut t);
        let fmt = "time and date: %r, %a %b %d, %Y";
        if strftime(buf1.as_char(), 16, fmt.to_ptr(), tmp) == 0 {
            printf("buffer length 16 is too small\n".to_ptr());
        } else {
            printf("%s\n".to_ptr(), buf1.as_ptr());
        }
        if strftime(buf2.as_char(), 64, fmt.to_ptr(), tmp) == 0 {
            printf("buffer length 64 is too small\n".to_ptr());
        } else {
            printf("%s\n".to_ptr(), buf2.as_ptr());
        }
        exit(0);
    }
}
