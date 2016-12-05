/// Exercise 6.3: Write a program that calls uname and prints all the fields
/// in the utsname structure.
/// Compare the output to the output from the uname(1) command.

extern crate libc;
extern crate apue;

use libc::{utsname, uname};
use apue::array_to_string;

#[derive(Debug)]
struct UtsName {
    sysname:  String,
    nodename: String,
    release:  String,
    version:  String,
    machine:  String,
}

unsafe fn my_uname() -> Option<UtsName> {
    let mut uc: utsname = std::mem::uninitialized();
    if uname(&mut uc) == 0 {
        return Some(UtsName {
            sysname: array_to_string(&uc.sysname).to_owned(),
            nodename: array_to_string(&uc.nodename).to_owned(),
            release: array_to_string(&uc.release).to_owned(),
            version: array_to_string(&uc.version).to_owned(),
            machine: array_to_string(&uc.machine).to_owned(),
        });
    }
    None
}


fn main() {
    println!("{:?}", unsafe { my_uname().unwrap() } );
}

// Result:
//
// > uname -a
// Darwin philippkellr-6.local 15.6.0 Darwin Kernel Version 15.6.0:
// Mon Aug 29 20:21:34 PDT 2016;
// root:xnu-3248.60.11~1/RELEASE_X86_64 x86_64
//
// > target/debug/e06-03-uname
// UtsName { sysname: "Darwin",
//           nodename: "philippkellr-6.local",
//           release: "15.6.0",
//           version: "Darwin Kernel Version 15.6.0: Mon Aug 29 20:21:34 PDT 2016;
//                     root:xnu-3248.60.11~1/RELEASE_X86_64",
//           machine: "x86_64" }
//
// -> it has the same infos, uname -a concatenates the fields with a space inbetween
