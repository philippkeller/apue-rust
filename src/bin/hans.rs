extern crate libc;

use std::ffi::CStr;
use std::io;
use std::str;

static BUF_BYTES: usize = 4096;

fn main() {
	let buf = unsafe {
	    let mut buf = Vec::with_capacity(BUF_BYTES);
	    let ptr = buf.as_mut_ptr() as *mut libc::c_char;
	    if libc::getcwd(ptr, buf.capacity()).is_null() {
	    	panic!(io::Error::last_os_error());
	    }
		CStr::from_ptr(ptr).to_bytes()
	};
	println!("result: {}", str::from_utf8(buf).unwrap());
}