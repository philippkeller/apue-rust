extern crate libc;

use std::ffi::{CStr,CString};
use std::io;
use std::str;

static BUF_BYTES: usize = 4096;
pub const _PC_PATH_MAX: libc::c_int = 5; // constant is missing in libc

fn main() {
	unsafe {
		let root = CString::new(".").unwrap().as_ptr();
		let path_max = libc::pathconf(root, _PC_PATH_MAX);
		println!("PATH_MAX={}", path_max);
		loop {
			let path = CString::new("a").unwrap().as_ptr();
			libc::mkdir(path, libc::S_IRWXU);
			libc::chdir(path);
			let buf = {
			    let mut buf = Vec::with_capacity(BUF_BYTES);
			    let ptr = buf.as_mut_ptr() as *mut libc::c_char;
			    if libc::getcwd(ptr, buf.capacity()).is_null() {
			    	panic!(io::Error::last_os_error());
			    }
				CStr::from_ptr(ptr).to_bytes()
			};
			println!("path length: {}, max path: {}", str::from_utf8(buf).unwrap().len(), path_max);
		}
	}
}

// # Solutions:

// - With the getcwd in place the max directory size is 1024 (MAX_PATH)
//   even when provided with a 4096 byte buffer.
// - Without getcwd there seems to be no limit