extern crate libc;

use libc::{c_int, c_char};

#[macro_export]
macro_rules! cstr {
    ($s:expr) => {{ 
        use std::ffi::CString;
        CString::new($s).unwrap().as_ptr() 
    }}
}

pub trait LibcResult<T> {
    /// returns None if the `c_int` is below 0, and Some otherwise
    ///
    /// # Example
    /// if let Some(fd) = libc::creat(fd1, FILE_MODE).to_option() {
    ///     fd
    /// } else {
    ///     panic!("{}", io::Error::last_os_error());
    /// }
    fn to_option(&self) -> Option<T>;
}

impl LibcResult<c_int> for c_int {
    fn to_option(&self) -> Option<c_int> {
        if *self < 0 { None } else { Some(*self) }
    }
}

pub trait CArray {
    fn as_char(&self) -> *mut c_char;
}

impl CArray for [c_char] {
    fn as_char(&self) -> *mut c_char {
        self.as_ptr() as *mut c_char
    }
}