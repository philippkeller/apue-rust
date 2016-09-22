/// Figure 6.2: The getpwname function
///
/// Takeaways: 
/// - from the book (p. 186): "the get functions return a pointer to a static structure, 
///   so we always have to copy the structure if we want to save it.", although in the
///   C example code they just return the struct so I'd say that's broken. We need
///   to copy the struct. Unfortunately, there's no cheap way to own the struct returned
///   by getpwent(). We need to build PasswdOwned to own the name string.
/// - originally used CString::from_raw on pw.pw_name, this worked well on OSX
///   but segfaulted on Linux. CStr::from_ptr needs to be called on Strings that originate
///   in C


extern crate libc;

use libc::{getpwent, endpwent, setpwent};
use std::ffi::CStr;

#[derive(Debug)]
struct PasswdOwned {
    name: String,
    uid: libc::gid_t,
    gid: libc::uid_t,
}

unsafe fn getpwnam(name: &str) -> Option<PasswdOwned> {
    setpwent();
    while let Some(pw) = getpwent().as_ref() {
        let pw_name = CStr::from_ptr(pw.pw_name).to_string_lossy().into_owned();
        if pw_name == name {
            endpwent();
            let pw = PasswdOwned{name: pw_name, uid: pw.pw_uid, gid: pw.pw_gid};
            return Some(pw);
        }
    }
    endpwent();
    None
}

fn main() {
    unsafe {
        println!("{:?}", getpwnam("philipp").expect("no user found with that name!"));
    }
}