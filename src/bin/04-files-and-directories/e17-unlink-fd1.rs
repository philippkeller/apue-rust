/// Exercise 4.17:
///
/// In Section 3.16, we described the /dev/fd feature. For any user to be able to access these
/// files, their permissions must be rw-rw-rw-.
/// Some programs that create an output file delete the file first, in case it already exists,
/// ignoring the return code:
///   unlink(path);
///   if ((fd = creat(path, FILE_MODE)) < 0)
///       err_sys(...);
///
/// What happens if path is /dev/fd/1?
///
/// $ e17-unlink-fd1
/// Operation not permitted (os error 1)
/// new fd: 3

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use std::io::Error;
use apue::LibcResult;
use libc::{S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP, S_IROTH, S_IWOTH, mode_t, creat, unlink};

const FILE_MODE: mode_t = S_IRUSR+S_IWUSR+S_IRGRP+S_IWGRP+S_IROTH+S_IWOTH;

fn main() {
    let fd = unsafe {
        if unlink(cstr!("/dev/fd/1")) < 0 {
            println!("{}", Error::last_os_error());
        }
        if let Some(fd) = creat(cstr!("/dev/fd/1"), FILE_MODE).to_option() {
            fd
        } else {
            panic!("{}", Error::last_os_error());
        }
    };
    println!("new fd: {}", fd);
}