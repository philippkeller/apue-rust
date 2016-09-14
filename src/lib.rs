extern crate libc;
use std::ffi::CString;


pub trait ToPtr {
    fn to_ptr(&self) -> *const i8;
}

impl ToPtr for str {
    fn to_ptr(&self) -> *const i8 {
        CString::new(self).unwrap().as_ptr()
    }
}

pub trait LibcIntResult {
    /// returns None if the `c_int` is below 0, and Some otherwise
    ///
    /// # Example
    /// if let Some(fd) = libc::creat(fd1, FILE_MODE).to_option() {
    ///     fd
    /// } else {
    ///     panic!("{}", io::Error::last_os_error());
    /// }
    fn to_option(&self) -> Option<libc::c_int>;
}

impl LibcIntResult for libc::c_int {
    fn to_option(&self) -> Option<libc::c_int> {
        if *self < 0 { None } else { Some(*self) }
    }
}
