/// Figure 2.17: Determine the number of file descriptors

/// Takeaway: On OSX this outputs 256 which is way too low.
/// `sysctl kern.maxfiles` returns 12288 and `kern.maxfilesperproc` returns 10240.
/// OPEN_MAX is defined in /usr/include/sys/syslimits.h as 10240 but I didn't find a way
/// how to reference that (AFAIK extern blocks can only reference functions, not constants)
/// On Linux it returns 1024 (which matches `ulimit -n`)
///
/// $ f17-open-max | grep succeeded
/// sysconf succeeded..

extern crate libc;
extern crate errno;
extern crate apue;

use libc::{_SC_OPEN_MAX, sysconf};
use errno::errno;
use apue::LibcResult;

const OPEN_MAX_GUESS: i64 = 256;

unsafe fn open_max(openmax: &mut i64) -> i64 {
    if *openmax == 0 {
        *openmax = match sysconf(_SC_OPEN_MAX).check_not_negative() {
            Ok(val) => {
                println!("sysconf succeeded..");
                val
            }
            Err(_) => {
                println!("sysconf failed..");
                match errno().0 {
                    0 => OPEN_MAX_GUESS, // indeterminate so just a guess
                    _ => panic!("pathconf error for _PC_PATH_MAX"),
                }
            }
        }
    }
    *openmax
}

fn main() {
    let mut openmax = 0;
    unsafe {
        println!("{:?}", open_max(&mut openmax));
    }
}
