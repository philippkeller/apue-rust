/// Exercise 6.3: Write a program that calls uname and prints all the fields
/// in the utsname structure.
/// Compare the output to the output from the uname(1) command.
///
/// Takeaway: couldn't find a method in CStr or CString
///           this came close:
///           https://gist.github.com/philippkeller/89a8a0b47362e86570958dc7a14e84d7
///           but produced Err(FromBytesWithNulError { _a: () })

extern crate libc;
extern crate itertools;

use libc::{utsname, uname};
use itertools::Itertools;

fn array_to_string(slice: &[i8]) -> String {
    slice.iter().take_while(|&x| *x != 0).map(|&a| a as u8 as char).join("")
}

#[derive(Debug)]
struct UtsName {
    sysname: String,
    nodename: String,
    release: String,
    version: String,
    machine: String,
}

fn my_uname() -> Option<UtsName> {
    let mut uc: utsname = unsafe { std::mem::uninitialized() };
    if unsafe { uname(&mut uc) } == 0 {
        return Some(UtsName {
            sysname: array_to_string(&uc.sysname),
            nodename: array_to_string(&uc.nodename),
            release: array_to_string(&uc.release),
            version: array_to_string(&uc.version),
            machine: array_to_string(&uc.machine),
        });
    }
    None
}


fn main() {
    println!("{:?}", my_uname().unwrap());
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
