extern crate libc;

use std::ffi::CStr;

extern {
    pub fn tmpnam(ptr: *mut libc::c_char) -> *mut libc::c_char;
}

fn main() {
	unsafe {
		let tmp_ptr = tmpnam(std::ptr::null_mut());
		let tmp = CStr::from_ptr(tmp_ptr).to_owned();
		println!("{}", tmp.into_string().unwrap());
	}
}