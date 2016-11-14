/// Figure 3.2: Create a file with a hole in it

/// Takeaway: On OSX the sparse file seems to use the full disk space:
///
/// > ./f02-file-with-hole
/// > cat file.hole > file.nohole
/// > ls -ls file.hole file.nohole
/// 40 -rw-r--r--  1 philipp  staff  16394 Nov 14 08:10 file.hole
/// 40 -rw-r--r--  1 philipp  staff  16394 Nov 14 08:10 file.nohole
///
/// Apperently the sparse files are not using disk space but ls -s is not able to spot this,
/// here's some explanation: http://superuser.com/questions/199109
/// Even in Virtual Box on OSX this is applying, so this seems to be a "feature" of HFS.
/// On a Linux host running on ext4 ls -ls shows 8 for file.hole and 40 for file.nohole

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
