extern crate libc;
extern crate apue;

use std::ffi::CString;
use std::io;
use apue::LibcResult;

static FILE_MODE: libc::mode_t = libc::S_IRUSR + libc::S_IWUSR + libc::S_IRGRP + libc::S_IWGRP +
                                 libc::S_IROTH + libc::S_IWOTH;

fn main() {
    let fd1 = CString::new("/dev/fd/1").unwrap().as_ptr();
    let fd = unsafe {
        if libc::unlink(fd1) < 0 {
            println!("{}", io::Error::last_os_error());
        }
        if let Some(fd) = libc::creat(fd1, FILE_MODE).to_option() {
            fd
        } else {
            panic!("{}", io::Error::last_os_error());
        }
    };
    println!("{}", fd);
}

// # Solution:

// on OS X: 'Permission denied' when unlinking /dev/fd/1
//           creating /dev/fd/1 results in a new fd with value 3
