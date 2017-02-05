/// Exercise 10.10: Write a program that calls sleep(60) in an infinite loop.
/// Every five times through the loop (every 5 minutes), fetch the current
/// time of day and print the tm_sec field. Run the program overnight and
/// explain the results. How would a program such as the cron daemon,
/// which runs every minute on the minute, handle this situation?

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{sleep, time, localtime, c_char};
use std::mem::uninitialized;
use std::ffi::CStr;
use apue::my_libc::strftime;

fn main() {
    unsafe {
        let mut buf: [c_char; 256] = uninitialized();
        let mut t = uninitialized();
        loop {
            sleep(60);
            time(&mut t);
            let tm = localtime(&t);
            strftime(buf.as_mut_ptr(), 256, cstr!("%a %b %e %H:%M:%S %Z %Y"), tm);
            println!("{:?}, tm_sec={}",
                     CStr::from_ptr(buf.as_ptr()),
                     (*tm).tm_sec);
        }
    }
}
