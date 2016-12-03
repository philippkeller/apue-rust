/// Figure 4.25: Print st_dev and st_rdev values
///
/// takeway: `minor()` and `major()` are static inline functions in C and cannot be called
/// from rust -> needed to reimplement them in `lib.rs`
///
/// $ f25-st_dev > /dev/null # device numbers are different on ev. machine -> only test ret code

extern crate libc;
extern crate apue;

use std::env::args;
use libc::{S_IFMT, S_IFCHR, S_IFBLK, stat};
use std::ffi::CString;
use apue::{err_sys, major, minor, LibcResult};

fn main() {
    let mut ar = args();
    ar.next();
    let mut buf:stat = unsafe {std::mem::uninitialized()};
    while let Some(a) = ar.next() {
        print!("{}: ", a);
        if let None = unsafe{stat(CString::new(a).unwrap().as_ptr(), &mut buf)}.to_option() {
            err_sys("stat error");
            continue;
        }
        print!("dev = {}/{}", major(buf.st_dev), minor(buf.st_dev));
        match buf.st_mode & S_IFMT {
            S_IFCHR | S_IFBLK => {
                let s = if buf.st_mode & S_IFMT == S_IFCHR {
                    "character"
                } else {
                    "block"
                };
                print!(" ({}) rdev = {}/{}", s, major(buf.st_rdev), minor(buf.st_rdev));
            },
            _ => {},
        }
        print!("\n");
    }
}