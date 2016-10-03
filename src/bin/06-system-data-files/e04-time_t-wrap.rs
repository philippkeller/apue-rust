/// Exercise 6.4 Calculate the latest time that can be represented by the time_t data type.
/// After it wraps around, what happens?

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{tm, time_t, c_char, size_t, printf};
use std::mem::uninitialized;

use apue::CArray;

extern "C" {
    fn localtime(time: *const time_t) -> *mut tm;
    fn strftime(s: *mut c_char,
                maxsize: size_t,
                format: *const c_char,
                timeptr: *const tm)
                -> size_t;
}

fn main() {
    unsafe {
        let buf: [c_char; 1024] = uninitialized();
        let mut t: time_t = 1;
        loop {
            println!("{:?}", t - 1);
            strftime(buf.as_char(),
                     1024,
                     cstr!("%a %b %d, %Y"),
                     localtime(&mut (t - 1)));
            printf(cstr!("%s\n"), buf.as_ptr());
            t *= 2;
        }
    }
}

// Answer: there's a Segmentation fault on strftime. This probably happens when the year
// within tm is bigger than 2147483647 (max i32 value). There's no "wrap around". Either
// the question is intentionally misleading or there are some systems where it wraps
// around. On OSX or Linux it doesn't.
