/// Figure 11.5 Thread cleanup handler
///
/// Status: does not compile yet,
/// opened http://stackoverflow.com/questions/42284562

extern crate libc;

use std::ffi::CStr;
use libc::{c_void, c_char};
use libc::pthread_create;
use std::ptr::null_mut;

fn cleanup(arg:*const c_void) {
    unsafe {
        let s = CStr::from_ptr(arg as _);
        println!("cleanup: {:?}", s);
    }
}

#[link(name = "thread-cleanup.o")]
extern "C" {
    fn thr_fn1(arg:*mut c_void) -> *mut c_void;
//    fn thr_fn2(arg: *const c_void) -> *mut c_void;
}

fn main() {
    let mut tid1 = std::mem::zeroed();
    libc::pthread_create(&mut tid1, null_mut(), thr_fn1, null_mut());
}