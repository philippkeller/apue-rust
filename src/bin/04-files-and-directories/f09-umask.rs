/// Figure 4.9 Example of umask function
///
/// Takeaway: formatting stat output is totally different on linux and on macos
///
/// $ rm /tmp/{foo,bar}
///
/// linux only:
/// $ f09-umask
/// $ stat -c %A /tmp/{foo,bar}
/// -rw-rw-rw-
/// -rw-------
/// $ rm /tmp/{foo,bar}
///
/// mac only:
/// $ f09-umask
/// $ stat -f "%Sp" /tmp/{foo,bar}
/// -rw-rw-rw-
/// -rw-------
/// $ rm /tmp/{foo,bar}

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{mode_t,S_IRUSR,S_IWUSR,S_IRGRP,S_IWGRP,S_IROTH,S_IWOTH, umask, creat};
use apue::LibcResult;

const RWRWRW:mode_t = (S_IRUSR|S_IWUSR|S_IRGRP|S_IWGRP|S_IROTH|S_IWOTH);

fn main() {
    unsafe {
        umask(0);
        if let None = creat(cstr!("/tmp/foo"), RWRWRW).to_option() {
            panic!("creat error for /tmp/foo");
        }
        umask(S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH);
        if let None = creat(cstr!("/tmp/bar"), RWRWRW).to_option() {
            panic!("creat error for /tmp/bar");
        }
    }
}