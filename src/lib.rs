extern crate libc;
extern crate itertools;
extern crate errno;

use libc::{c_int, c_char, dev_t, utsname, exit, PATH_MAX};
use std::io::Write;
use std::ffi::{CStr};

/// Turns a str into a c string. Warning: the cstring only lives as long the
/// str lives. Don't e.g. assign the return value to a variable!
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {{
        use std::ffi::CString;
        CString::new($s).unwrap().as_ptr()
    }}
}

#[macro_export]
macro_rules! as_void {
    ($s:expr) => {{
        extern crate libc;
        use libc::c_void;
        $s.as_ptr() as *mut c_void
    }}
}

#[macro_export]
macro_rules! as_char {
    ($s:expr) => {{
        extern crate libc;
        use libc::c_char;
        $s.as_ptr() as *mut c_char
    }}
}

#[macro_export]
macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;
            if let Err(e) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
                panic!("Failed to write to stderr.\
                    \nOriginal error output: {}\
                    \nSecondary error writing to stderr: {}", format!($($arg)*), e);
            }
        }
    )
}

/// turn libc result into an option
pub trait LibcResult<T> {
    /// returns None if the result is empty (-1 if an integer, Null if a pointer)
    /// and Some otherwise
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
impl LibcResult<i64> for i64 {
    fn to_option(&self) -> Option<i64> {
        if *self < 0 { None } else { Some(*self) }
    }
}

// implementation for isize, sentinel = 0 (means end of file/buffer/... e.g. in read)
impl LibcResult<isize> for isize {
    fn to_option(&self) -> Option<isize> {
        if *self <= 0 { None } else { Some(*self) }
    }
}
impl<T> LibcResult<*mut T> for *mut T {
    fn to_option(&self) -> Option<*mut T> {
        if self.is_null() { None } else { Some(*self) }
    }
}

pub unsafe fn array_to_string(sl: &[i8]) -> &str {
    CStr::from_ptr(sl.as_ptr()).to_str().expect("invalid string")
}

/// Return uname -s
pub fn uname() -> Option<String> {
    let mut uc: utsname = unsafe { std::mem::uninitialized() };
    unsafe {
        if libc::uname(&mut uc) == 0 {
            return Some(String::from(array_to_string(&uc.sysname)));
        }
    }
    None
}

pub fn err_sys(msg: &str) {
    std::io::stderr().write(format!("{}{}", msg, "\n").as_bytes()).unwrap();
    unsafe {
        exit(1);
    }
}

pub fn path_alloc() -> std::vec::Vec<c_char> {
    // Before POSIX.1-2001, we aren’t guaranteed that PATH_MAX includes
    // the terminating null byte. For simplicity sake we don't check for the posix
    // version and just increase by one
    Vec::with_capacity((PATH_MAX + 1) as usize)
}
// major device number, impl ported from /usr/include/sys/types.h
pub fn major(x:dev_t) -> dev_t {
    (x >> 24) & 0xff
}

// minor device number, impl ported from /usr/include/sys/types.h
pub fn minor(x:dev_t) -> dev_t {
    x & 0xffffff
}

pub mod my_libc {
    use libc::{dirent, DIR};
    extern "C" {
        #[cfg(target_os = "macos")]
        #[link_name = "readdir$INODE64"]
        pub fn readdir(arg1: *mut DIR) -> *mut dirent;

        #[cfg(not(target_os = "macos"))]
        pub fn readdir(arg1: *mut DIR) -> *mut dirent;

    }
}