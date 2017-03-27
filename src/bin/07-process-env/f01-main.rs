#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![no_main]

/// Figure 7.1: Classic C program
///
/// Takeaway: per default rust creates a shim around the main. There's e.g. no way
/// to return an int from the main method. Luckily it's quite easy to remove it via
/// no-stdlib: https://doc.rust-lang.org/book/no-stdlib.html
/// I didn't need all the options, e.g. I didn't need to specify default-features = false
/// for libc
///
/// fyi: Exit code 13 comes from the return code of printf (see answer to Exercise 7.1)
/// update: before the exit code seemed stable, now it is somehow random. Got 10 on OSX
/// and 126 on Linux
///
// $ f01-main | cat # cat is needed so it "swallows" the exit code from f01-main
// Hello World!

extern crate libc;

use libc::printf;

#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) {
    unsafe {
        printf("Hello World!\n\0".as_ptr() as *const i8);
    }
}


// This is needed for Linux but not for Mac
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern "C" fn rust_eh_unwind_resume() {}
