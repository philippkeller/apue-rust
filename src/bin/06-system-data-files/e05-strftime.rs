/// Exercise: Write a program to obtain the current time and print it using strftime,
/// so that it looks like the default output from date(1). Set the TZ environment
/// variable to different values and see what happens.

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{tm, time_t, c_char, size_t, printf};
use std::mem::uninitialized;

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
        let mut buf: [c_char; 256] = uninitialized();
        let mut t: time_t = uninitialized();
        time(&mut t);
        strftime(buf.as_mut_ptr(),
                 256,
                 cstr!("%a %b %e %H:%M:%S %Z %Y"),
                 localtime(&t));
        printf(cstr!("%s\n"), buf.as_ptr());
    }
}

// ## Results
//
// $ export TZ='Europe/Zurich'
// $ macos/target/debug/e05-strftime
// Sun Oct  2 14:59:33 CEST 2016
//
// $ export TZ='America/Toronto'
// $ macos/target/debug/e05-strftime
// Sun Oct  2 09:00:50 EDT 2016
