/// Figure 11.5 Thread cleanup handler
///
/// Findings:
/// - libc function definition was wrong, opened http://stackoverflow.com/questions/42284562
///   and PR for libc: https://github.com/rust-lang/libc/pull/527
/// - as pthread_cleanup_push and pthread_cleanup_pop are implemented as macros and rust doesn't
///   allow the same style of macros this part needed to stay in C. I tried integrating Rust
///   and C (calls going into both directions), using `my_build.rs` and the gcc module.
///   But: With this addition, any change in any rust file caused a recompilation of the whole
///   project, opened https://github.com/rust-lang/cargo/issues/3724 and added
///   `cargo:rerun-if-changed=` into my_build.rs which now causes only a project rebuild if
///   my_build.rs is changed or `thread_cleanup.c`
///
/// The program behaves as described in the book:
///
/// mac only:
/// $ f05-thread-cleanup 2>&1
/// thread 1 start
/// thread 1 push complete
/// thread 2 start
/// thread 2 push complete
/// cleanup: "thread 1 second handler"
/// cleanup: "thread 2 second handler"
/// cleanup: "thread 2 first handler"
/// ERROR: return code 139
///
/// linux only:
/// $ f05-thread-cleanup 2>&1
/// thread 1 start
/// thread 1 push complete
/// thread 2 start
/// thread 2 push complete
/// thread 1 exit code: 0x1
/// cleanup: "thread 2 second handler"
/// cleanup: "thread 2 first handler"
/// thread 2 exit code: 0x2

extern crate libc;
extern crate apue;

use std::ffi::CStr;
use libc::c_void;
use libc::{pthread_join, usleep};
use std::ptr::null_mut;
use apue::my_libc::pthread_create;
use apue::PthreadExpect;

#[no_mangle]
pub extern "C" fn cleanup(arg: *mut c_void) {
    unsafe {
        let s = CStr::from_ptr(arg as _);
        println!("cleanup: {:?}", s);
    }
}


#[link(name = "thread-cleanup")]
extern "C" {
    fn thr_fn1(arg: *mut c_void) -> *mut c_void;
    fn thr_fn2(arg: *mut c_void) -> *mut c_void;
}


fn main() {
    unsafe {
        let (mut tid1, mut tid2) = std::mem::zeroed();
        let mut tret = std::mem::uninitialized();
        pthread_create(&mut tid1, null_mut(), thr_fn1, 1 as _).expect("can't create thread 1");
        usleep(10);
        pthread_create(&mut tid2, null_mut(), thr_fn2, 1 as _).expect("can't create thread 2");
        pthread_join(tid1, &mut tret).expect("can’t join with thread 1");
        println!("thread 1 exit code: {:?}", tret);
        pthread_join(tid2, &mut tret).expect("can’t join with thread 2");
        println!("thread 2 exit code: {:?}", tret);
    }
}