#![allow(unused_variables)]

/// BSD-based systems provide a function called funopen that
/// allows us to intercept read, write, seek, and close calls
/// on a stream. Use this function to implement fmemopen
/// for FreeBSD and Mac OS X.

extern crate libc;

use libc::{c_void, c_char, c_int, fpos_t, FILE, memset, memcpy, fgets};
use std::option::Option;

const BUFLEN: usize = 10;

#[derive(Debug)]
pub struct MemStream {
    buffer: [u8;BUFLEN],
    pos: u8
}

extern "C" {
     fn funopen(
     			cookie: *mut c_void, 
     			readfn: Option<unsafe extern "C" fn (
     				cookie: *mut c_void,
     				buffer: *mut c_char,
     				nbyte: c_int) -> c_int>,
     			writefn: Option<unsafe extern "C" fn (
     				cookie: *mut c_void,
     				buffer: *mut c_char,
     				nbyte: c_int) -> c_int>,
     			seekfn: Option<unsafe extern "C" fn (
     				cookie: *mut c_void,
     				offset: fpos_t,
     				whence: c_int) -> fpos_t>,
     			closefn: Option<unsafe extern "C" fn (
     				cookie: *mut c_void
     				) -> c_int>
     			) -> *mut FILE;
}

unsafe extern "C" fn read(cookie: *mut c_void, buffer: *mut c_char, nbyte: c_int) -> c_int {
    // println!("{:?}", cookie.read(5));
    let status = cookie as *mut MemStream;
    println!("position is: {:?}", (*status).pos);
    5
}

unsafe extern "C" fn write(cookie: *mut c_void, buffer: *mut c_char, nbyte: c_int) -> c_int {
    println!("write was called!");
    5
}

unsafe extern "C" fn seek(cookie: *mut c_void, offset: fpos_t, whence: c_int) -> fpos_t {
    println!("seek was called!");
    let a:fpos_t = std::mem::uninitialized();
    a
}

unsafe fn fmemopen(cookie: *mut c_void) -> *mut FILE {
   funopen(cookie, Some(read), Some(write), Some(seek), None)
}

fn main() {
    unsafe {
        let mut buf: [u8; BUFLEN] = std::mem::uninitialized();
        memset(buf.as_ptr() as *mut c_void, 'a' as c_int, BUFLEN - 2);
        buf[BUFLEN - 2] = '\0' as u8;
        buf[BUFLEN - 1] = 'X' as u8;
        let mut status = MemStream { buffer: buf, pos: 0 };
        // could be written as &mut status as *mut _ as *mut _
        // so the types would be coerced but for clarity I wrote
        // out the types. Two casts are neeeded because first
        // the reference is cast into a pointer and then the pointer
        // type is changed
        let fd = fmemopen(&mut status as *mut MemStream as *mut c_void);

        let tmpbuf: [u8; 5] = std::mem::uninitialized();
        fgets(tmpbuf.as_ptr() as *mut i8, 5, fd);

    }
}
