/// Excercise 6.1: If the system uses a shadow file and we need to
///                obtain the encrypted password, how do we do so?
///
/// Takeaways
///
/// - bindgen is great, spend time to get it working instead of trying to come
///   up with the bindings yourself
/// - user needs to be root, checking for root with getuid
/// - first tried with iterating via getspent, then saw getspnam which is a lot easier of course

extern crate libc;
#[cfg(target_os = "linux")]
use std::ffi::{CStr, CString};
#[cfg(target_os = "linux")]
use libc::{getuid, c_char, c_long, c_ulong};

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
#[cfg(target_os = "linux")]
pub struct spwd {
    pub sp_namp: *mut c_char,
    pub sp_pwdp: *mut c_char,
    pub sp_lstchg: c_long,
    pub sp_min: c_long,
    pub sp_max: c_long,
    pub sp_warn: c_long,
    pub sp_inact: c_long,
    pub sp_expire: c_long,
    pub sp_flag: c_ulong,
}

#[cfg(target_os = "linux")]
extern "C" {
    pub fn setspent();
    pub fn endspent();
    pub fn getspent() -> *mut spwd;
    pub fn getspnam(__name: *const ::std::os::raw::c_char) -> *mut spwd;
}

#[derive(Debug)]
#[cfg(target_os = "linux")]
struct PasswdOwned {
    name: String,
    pw: String,
}

#[cfg(target_os = "linux")]
unsafe fn getpwnam_iter(name: &str) -> Option<PasswdOwned> {
    setspent();
    while let Some(pw) = getspent().as_ref() {
        let pw_name = CStr::from_ptr(pw.sp_namp).to_string_lossy().into_owned();
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

#[cfg(target_os = "linux")]
unsafe fn getpwnam(name: &str) -> Option<PasswdOwned> {
    match getspnam(CString::new(name).unwrap().as_ptr()).as_ref() {
        Some(pw) => {
            Some(PasswdOwned {
                name: CStr::from_ptr(pw.sp_namp).to_string_lossy().into_owned(),
                pw: CStr::from_ptr(pw.sp_pwdp).to_string_lossy().into_owned(),
            })
        }
        None => None,
    }
}

#[cfg(any(target_os = "linux"))]
fn main() {
    unsafe {
        if getuid() != 0 {
            panic!("you need to start as root, e.g. via sudo");
        }
        println!("{:?}",
                 getpwnam_iter("philipp").expect("no user found with that name!"));
        println!("{:?}",
                 getpwnam("philipp").expect("no user found with that name!"));
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {}
