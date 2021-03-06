/// Figure 2.16 Dynamically allocate space for a pathname

/// Takeaway: in the C example in the book the return value of pathconf is increased
/// by one "since it’s relative to root" but both on OSX and Linux I don't need to do that.
/// A short online research also showed that _PC_PATH_MAX is not relative to the given path
/// but the absolute pathmax value for the filesystem where the path points to.
///
/// mac only:
/// $ f16-pathname-alloc-space
/// from libc constant: PATH_MAX=1024
/// from pathconf: pathmax = 1024
/// length of pointer = 1024
///
/// linux only:
/// $ f16-pathname-alloc-space
/// from libc constant: PATH_MAX=4096
/// from pathconf: pathmax = 4096
/// length of pointer = 4096

extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;

use libc::{_SC_VERSION, _SC_XOPEN_VERSION, PATH_MAX, _PC_PATH_MAX, sysconf, pathconf, malloc};
use apue::{LibcResult, LibcPtrResult};
use errno::errno;

const PATH_MAX_GUESS: i64 = 1024;

unsafe fn path_alloc(pathmax: &mut i64,
                     posix_version: &mut i64,
                     xsi_version: &mut i64)
                     -> (*mut libc::c_void, i64) {
    if *posix_version == 0 {
        if let Ok(val) = sysconf(_SC_VERSION).check_not_negative() {
            *posix_version = val;
        }
    }
    if *xsi_version == 0 {
        if let Ok(val) = sysconf(_SC_XOPEN_VERSION).check_not_negative() {
            *xsi_version = val;
        }
    }
    println!("from libc constant: PATH_MAX={:?}", PATH_MAX);
    // would be too easy to just take the constant so we go on..
    if *pathmax == 0 {
        *pathmax = if let Ok(val) = pathconf(cstr!("/"), _PC_PATH_MAX).check_not_negative() {
            val
        } else {
            let e = errno();
            match e.0 {
                0 => PATH_MAX_GUESS, // indeterminate so just a guess
                _ => panic!("pathconf error for _PC_PATH_MAX"),
            }
        }
    }
    println!("from pathconf: pathmax = {:?}", *pathmax);

    // Before POSIX.1-2001, we aren’t guaranteed that PATH_MAX includes
    // the terminating null byte.  Same goes for XPG3.
    let size = if *posix_version < 200112 && *xsi_version < 4 {
        *pathmax + 1
    } else {
        *pathmax
    };
    if let Ok(ptr) = malloc(size as _).check_not_null() {
        (ptr, size)
    } else {
        panic!("malloc error for pathname");
    }
}

fn main() {
    let mut posix_version = 0;
    let mut xsi_version = 0;
    let mut pathmax = 0;
    unsafe {
        let (_, size) = path_alloc(&mut pathmax, &mut posix_version, &mut xsi_version);
        println!("length of pointer = {:?}", size);
    }
}
