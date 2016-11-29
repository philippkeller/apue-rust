/// Exercise 3.6: If you open a file for read–write with the append flag, can you still read
/// from anywhere in the file using lseek? Can you use lseek to replace existing data in the
/// file? Write a program to verify this.
///
/// Answer: yes you can
///
/// $ echo "123456789" > /tmp/e06.txt
/// $ e06-read-write-seek /tmp/e06.txt
/// $ cat /tmp/e06.txt
/// 123hansaplast!

extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;
#[macro_use(value_t)]
extern crate clap;

use libc::{SEEK_SET, fopen, fseek, fputs};
use errno::errno;
use apue::LibcResult;
use clap::App;
use std::ffi::CString;

fn main() {
    let matches = App::new("e06").args_from_usage("<file> path to file to be opened for read/write/seek").get_matches();
    let file = matches.value_of("file").unwrap();
    unsafe {
        if let Some(f) = fopen(CString::new(file).unwrap().as_ptr(), cstr!("r+")).to_option() {
            if let Some(_) = fseek(f, 3, SEEK_SET).to_option() {
                fputs(cstr!("hansaplast!") as _, f);
            } else {
                panic!("fseek exited with '{}'", errno());
            }
        } else {
            panic!("fopen exited with '{}'", errno());
        }

    }
}