#![allow(unused_imports)]

/// Figure 4.21 Example of futimens function
///
/// Takeaways:
///
/// - futimens only exists for linux, not for bsd based systems as osx
/// - in the rust struct `stat` the field `st_atim` is missing os we need
///   to fall back on `st_atime` and `st_atime_nsec`
/// - from the rust function definition of `futimens` it was not clear that the
///   function takes an array of size two
/// - there is no way of changing the ctime with commandline tools to the past
///   so we need to touch and then wait one second
///
/// linux only:
/// $ rm -f /tmp/f21.txt
/// $ touch /tmp/f21.txt
/// $ sleep 1
/// $ f21-futimes /tmp/f21.txt
/// $ a=$(date "+%s"); b=$(stat -c %Z /tmp/f21.txt); echo $(($a-$b))
/// 0

extern crate libc;
#[macro_use(cstr, print_err)]
extern crate apue;

use apue::LibcResult;

#[cfg(target_os = "linux")]
use libc::{O_RDWR, O_TRUNC, stat, timespec, open, futimens, close};
use std::ffi::CString;

#[cfg(target_os = "linux")]
fn main() {
    unsafe {
        let mut statbuf: stat = std::mem::uninitialized();
        let mut args = std::env::args();
        args.next(); // skip filename
        while let Some(filename) = args.next() {
            let filename = CString::new(filename).unwrap();
            if stat(filename.as_ptr(), &mut statbuf).check_not_negative().is_err() {
                print_err!("{:?}: stat error", filename);
            } else if let Ok(fd) = open(filename.as_ptr(), O_RDWR | O_TRUNC).check_not_negative() {
                let times: [timespec; 2] = [timespec {
                                                tv_sec: statbuf.st_atime,
                                                tv_nsec: statbuf.st_atime_nsec,
                                            },
                                            timespec {
                                                tv_sec: statbuf.st_mtime,
                                                tv_nsec: statbuf.st_mtime_nsec,
                                            }];
                // reset times
                if futimens(fd, times.as_ptr() as *const _).check_not_negative().is_err() {
                    print_err!("{:?}: futimens error", filename);
                }
                close(fd);
            } else {
                print_err!("{:?}: open error", filename);
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    unimplemented!();
}
