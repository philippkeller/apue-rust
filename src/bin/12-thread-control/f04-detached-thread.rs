/// Figure 12.4 Creating a thread in the detached state
///
/// Findings:
/// - finally needed to clean up error handling, totally rewrote what was former only a
///   to_option() method in lib.rs into several check_* methods, see also this discussion:
///   http://stackoverflow.com/questions/42772307
/// - detached threads on linux can't write to stdout, got
///   "failed printing to stdout: Broken pipe (os error 32)"
///
/// mac only:
/// $ f04-detached-thread
/// called!

extern crate libc;
extern crate apue;

use libc::{c_void, PTHREAD_CREATE_DETACHED};
use libc::{pthread_create, pthread_attr_destroy, pthread_attr_init, pthread_attr_setdetachstate,
           usleep};
use apue::LibcResult;

extern "C" fn my_thread(_: *mut c_void) -> *mut c_void {
    println!("called!");
    0 as _
}

unsafe fn makethread(func: extern "C" fn(*mut c_void) -> *mut c_void,
                     arg: *mut c_void)
                     -> std::io::Result<i32> {
    let (mut attr, mut tid) = std::mem::uninitialized();
    pthread_attr_init(&mut attr).check_zero()?;
    let mut err = 0;
    if pthread_attr_setdetachstate(&mut attr, PTHREAD_CREATE_DETACHED).check_zero().is_ok() {
        err = pthread_create(&mut tid, &attr, func, arg);
    }
    pthread_attr_destroy(&mut attr).check_zero()?;
    Ok(err)
}

fn main() {
    unsafe {
        makethread(my_thread, std::ptr::null_mut()).expect("couldn't do a thread");
        usleep(100);
    }
}
