/// Exercise 4.16: Dir tree depth
///
/// mac only:
/// $ e16-dir-tree-depth 2>/dev/null
/// PATH_MAX=1024
/// path length: 1022, max path: 1024
/// ERROR: return code 101
///
/// linux only:
/// $ e16-dir-tree-depth 2>/dev/null
/// PATH_MAX=4096
/// path length: 4094, max path: 4096
/// ERROR: return code 101

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use std::ffi::{CStr, CString};
use std::io;
use std::str;
use libc::{_PC_PATH_MAX, chdir, mkdir};

static BUF_BYTES: usize = 4096;
#[cfg(target_os = "macos")]
const GUESS_PATH_LENGH:usize = 1024;
#[cfg(target_os = "linux")]
const GUESS_PATH_LENGH:usize = 4096;

fn main() {
    unsafe {
        let path_max = libc::pathconf(cstr!("."), _PC_PATH_MAX);
        let initial = CString::new("/tmp/someinitialpathwhichisquitelongalreadysowedontneedtoloopforsolong").unwrap();
        mkdir(initial.as_ptr(), libc::S_IRWXU);
        chdir(initial.as_ptr());
        println!("PATH_MAX={}", path_max);
        loop {
            libc::mkdir(cstr!("a"), libc::S_IRWXU);
            libc::chdir(cstr!("a"));
            let buf = {
                let mut buf = Vec::with_capacity(BUF_BYTES);
                let ptr = buf.as_mut_ptr() as *mut libc::c_char;
                if libc::getcwd(ptr, buf.capacity()).is_null() {
                    panic!(io::Error::last_os_error());
                }
                let len = CStr::from_ptr(ptr).to_bytes().len();
                buf.set_len(len);
                CString::new(buf)
            };
            let s = buf.expect("Not a C string").into_string().expect("Not UTF-8");
            if s.len() > GUESS_PATH_LENGH - 4 {
                println!("path length: {}, max path: {}", s.len(), path_max);
            }
        }
    }
}

// # Solutions:

// - With the getcwd in place the max directory size is 1024 (MAX_PATH)
//   even when provided with a 4096 byte buffer.
// - Without getcwd there seems to be no limit
