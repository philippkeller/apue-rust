/// Figure 12.4 Creating a thread in the detached state
///
/// Error handling is still very ugly, I hope I soon get an answer here:
/// http://stackoverflow.com/questions/42772307
///
/// $ f04-detached-thread
/// called!

extern crate libc;
extern crate apue;

use libc::{c_void, PTHREAD_CREATE_DETACHED};
use libc::{pthread_create, pthread_attr_destroy, pthread_attr_init, pthread_attr_setdetachstate, usleep};
use apue::PthreadExpect;

extern "C" fn my_thread(_: *mut c_void) -> *mut c_void {
    println!("called!");
    0 as _
}

unsafe fn makethread(func: extern "C" fn(*mut c_void) -> *mut c_void, arg:*mut c_void) -> Result<i32, i32> {
    let (mut attr, mut tid) = std::mem::uninitialized();
    let err = pthread_attr_init(&mut attr).expect();
    if err > 0 {
        return Err(err);
    }
    let mut err = pthread_attr_setdetachstate(&mut attr, PTHREAD_CREATE_DETACHED);
    if err == 0 {
        err = pthread_create(&mut tid, &attr, func, arg);
    }
    pthread_attr_destroy(&mut attr);
    if err > 0 {
        Err(err)
    } else {
        Ok(0)
    }
}

fn main() {
    unsafe {
        makethread(my_thread, std::ptr::null_mut()).expect("couldn't do a thread");
        usleep(100);
    }
}