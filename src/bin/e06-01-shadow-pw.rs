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

use std::ffi::CStr;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct spwd {
    pub sp_namp: *mut ::std::os::raw::c_char,
    pub sp_pwdp: *mut ::std::os::raw::c_char,
    pub sp_lstchg: ::std::os::raw::c_long,
    pub sp_min: ::std::os::raw::c_long,
    pub sp_max: ::std::os::raw::c_long,
    pub sp_warn: ::std::os::raw::c_long,
    pub sp_inact: ::std::os::raw::c_long,
    pub sp_expire: ::std::os::raw::c_long,
    pub sp_flag: ::std::os::raw::c_ulong,
}

extern "C" {
    pub fn setspent();
    pub fn endspent();
    pub fn getspent() -> *mut spwd;
}

#[derive(Debug)]
struct PasswdOwned {
    name: String,
    pw: String,
}

unsafe fn getpwnam(name: &str) -> Option<PasswdOwned> {
    setspent();
    while let Some(pw) = getspent().as_ref() {
        let pw_name = CStr::from_ptr(pw.sp_namp).to_string_lossy().into_owned();
        println!("{:?}", pw_name);
        if pw_name == name {
            endspent();
            let pw = PasswdOwned {
                name: pw_name,
                pw: CStr::from_ptr(pw.sp_pwdp).to_string_lossy().into_owned(),
            };
            return Some(pw);
        }
    }
    endspent();
    None
}

fn main() {
    unsafe {
        println!("{:?}",
                 getpwnam("philipp").expect("no user found with that name!"));
    }
}
