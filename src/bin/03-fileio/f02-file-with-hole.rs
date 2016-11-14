/// Figure 3.2: Create a file with a hole in it

/// Takeaway: On OSX the sparse file seems to use the full disk space:
/// > ./f02-file-with-hole
/// > du -h file.hole
/// 20K   file.hole
///
/// Apperently the sparse files are not using disk space but du is not able to spot this,
/// here's some explanation: http://superuser.com/questions/199109

extern crate libc;
#[macro_use(cstr, as_void)]
extern crate apue;

use libc::{mode_t, SEEK_SET, creat, write, lseek};
use apue::LibcResult;
use std::ffi::CString;

const FILE_MODE: mode_t = (libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IROTH);

fn main() {
    unsafe {
        let s1 = CString::new("abcdefghij");
        let s2 = CString::new("ABCDEFGHIJ");
        let fd = creat(cstr!("file.hole"), FILE_MODE).to_option().expect("creat error");
        assert!(write(fd, as_void!(s1.unwrap().as_bytes()), 10) == 10,
                "buffer write error");
        // offset is now 10
        lseek(fd, 16384, SEEK_SET).to_option().expect("lseek error");
        assert!(write(fd, as_void!(s2.unwrap().as_bytes()), 10) == 10,
                "buffer write error");
        // offset is now 16384
    }
}
