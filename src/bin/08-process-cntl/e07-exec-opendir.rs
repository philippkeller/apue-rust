/// Exercise 8.7: We mentioned in Section 8.10 that POSIX.1 requires open directory streams to be
/// closed across an exec. Verify this as follows: call opendir for the root directory, peek at
/// your system’s implementation of the DIR structure, and print the close-on-exec flag.
/// Then open the same directory for reading, and print the close-on-exec flag.
///
/// From the doc: POSIX.1 specifically requires that open directory streams (recall the opendir
/// function from Section 4.22) be closed across an exec. This is normally done by the opendir
/// function calling fcntl to set the close-on-exec flag for the descriptor corresponding to
/// the open directory stream.
///
/// $ e07-exec-opendir
/// close-on-exec is set
/// close-on-exec is not set

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{opendir, fcntl, open, close, closedir, F_GETFD, FD_CLOEXEC, O_RDONLY};
use apue::LibcResult;
use apue::my_libc::{readdir, dirfd};

unsafe fn pr_flags(fd: i32) {
    let flags = fcntl(fd, F_GETFD);
    if flags & FD_CLOEXEC > 0 {
        println!("close-on-exec is set");
    } else {
        println!("close-on-exec is not set");
    }

}

fn main() {
    unsafe {
        let dp = opendir(cstr!("/"));
        let dfd = dirfd(dp);
        assert!(!dp.is_null(), "can't open root dir");
        if let Some(_) = readdir(dp).to_option() {
            // just read one entry and discard it
        }
        pr_flags(dfd);
        let fd = open(cstr!("/"), O_RDONLY).to_option().expect("cannot open root for reading");
        pr_flags(fd);
        close(fd);
        closedir(dp);
    }
}
