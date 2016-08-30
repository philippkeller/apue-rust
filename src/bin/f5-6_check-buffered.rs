#![allow(non_camel_case_types)]

/// Works for OS X only. To make this work on other platforms run 
/// bindgen on stdio.h and replace the bindgen generated code below
///
/// Main captcha here is that you first need to perform operations on
/// the stream before you can get any buffer information from it.

extern crate libc;
use std::ffi::CString;

// bindgen generaged code starts...
extern "C" {
    pub static mut __stdinp: *mut MY_FILE;
    pub static mut __stdoutp: *mut MY_FILE;
    pub static mut __stderrp: *mut MY_FILE;
}
pub type fpos_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
    _bindgen_padding_0_: [u8; 4usize],
}
pub enum __sFILEX { }
#[repr(C,)]
pub struct MY_FILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                               *mut ::std::os::raw::c_void)
                                          -> ::std::os::raw::c_int>,
    pub _read: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                              *mut ::std::os::raw::c_void,
                                                          arg2:
                                                              *mut ::std::os::raw::c_char,
                                                          arg3:
                                                              ::std::os::raw::c_int)
                                         -> ::std::os::raw::c_int>,
    pub _seek: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                              *mut ::std::os::raw::c_void,
                                                          arg2: fpos_t,
                                                          arg3:
                                                              ::std::os::raw::c_int)
                                         -> fpos_t>,
    pub _write: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                               *mut ::std::os::raw::c_void,
                                                           arg2:
                                                               *const ::std::os::raw::c_char,
                                                           arg3:
                                                               ::std::os::raw::c_int)
                                          -> ::std::os::raw::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}
// ... bindgen generated code ends

unsafe fn pr_stdio(name: &str, fp: *mut libc::FILE) {
    let fp = &mut *(fp as *mut MY_FILE);
    let buffer_type = 
      if (fp._flags & libc::_IONBF as i16) != 0 {
        "unbuffered"
      } else if (fp._flags & libc::_IOLBF as i16) != 0 {
        "line buffered"
      } else {
        "fully buffered"
      };

    println!("stream = {}, {}, buffer size = {}, fp = {}", name, buffer_type, fp._bf._size, fp._file);
}

fn main() {
    unsafe {
        let stdin = __stdinp as *mut libc::FILE;
        let stdout = __stdoutp as *mut libc::FILE;
        let stderr = __stderrp as *mut libc::FILE;
        let passwd = libc::fopen(b"/etc/passwd\0".as_ptr() as *const libc::c_char, b"r\0".as_ptr() as *const libc::c_char);
        libc::fputs(CString::new("enter any character\n").unwrap().as_ptr(), stdout);
        libc::getchar();
        libc::fputs(CString::new("one line to stderr\n").unwrap().as_ptr(), stderr);
        libc::fgetc(passwd);
        pr_stdio("stdin", stdin);
        pr_stdio("stdout", stdout);
        pr_stdio("stderr", stderr);
        pr_stdio("passwd", passwd);
    }    
}