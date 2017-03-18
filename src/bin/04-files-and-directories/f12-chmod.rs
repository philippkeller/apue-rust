/// Figure 4.12: Example of chmod function
///
/// Findings:
///
/// - setgid in /tmp/ does not work under OSX for whatever reason. With sudo it works
///   and in another directory and on Linux it all works. Cost me like 2h to track
///   it down (of course first tried to find the bug in the code)
///   full post: http://stackoverflow.com/questions/42811165
///
/// $ rm -f /tmp/{foo,bar}
/// $ touch /tmp/{foo,bar}
/// $ chmod g+x /tmp/foo
///
/// linux only:
/// $ f12-chmod
/// $ stat -c %A /tmp/{foo,bar}
/// -rwSrwxr--
/// -rw-r--r--
///
/// mac only:
/// $ f12-chmod
/// $ stat -f "%Sp" /tmp/{foo,bar}
/// -rwSr-xr--
/// -rw-r--r--

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{mode_t, S_IXUSR, S_ISUID, S_IROTH, S_IRGRP, S_IWUSR, S_IRUSR, stat, chmod};
use apue::{LibcResult, err_sys};

fn main() {
    unsafe {
        let mut statbuf: stat = std::mem::uninitialized();
        if let None = stat(cstr!("/tmp/foo"), &mut statbuf).to_option() {
            err_sys("stat error for /tmp/foo");
        }

        // turn on set-group-ID and turn off group-execute
        if let None = chmod(cstr!("/tmp/foo"),
                            (statbuf.st_mode & !S_IXUSR) | S_ISUID as mode_t)
            .to_option() {
            err_sys("chmod error for foo");
        }

        // set absolute mode to "rw-r--r--"
        if let None = chmod(cstr!("/tmp/bar"), S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH).to_option() {
            err_sys("chmod error for bar");
        }
    }
}
