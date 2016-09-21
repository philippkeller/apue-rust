/// Figure 6.2: The getpwname function
///
/// Takeaway: there's no cheap way to own the struct returned
/// by getpwent(). We need to build PasswdOwned to own the name string.

extern crate libc;

use libc::{getpwent, endpwent, setpwent};
use std::ffi::CString;

#[derive(Debug)]
struct PasswdOwned {
    name: String,
    uid: libc::gid_t,
    gid: libc::uid_t,
}

unsafe fn getpwnam(name: &str) -> Option<PasswdOwned> {
    setpwent();
    while let Some(pw) = getpwent().as_ref() {
        let pw_name = CString::from_raw(pw.pw_name).into_string().expect("found null");
        if pw_name == name {
            endpwent();
            return Some(PasswdOwned{name: pw_name, uid: pw.pw_uid, gid: pw.pw_gid});
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