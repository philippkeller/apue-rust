/// Figure 11.3 Fetching the thread exit status
///
/// $ f03-thread-exit
/// thread 2 returning
/// thread 1 returning
/// thread 1 exit code: 1
/// thread 2 exit code: 2

extern crate libc;
extern crate apue;
use libc::{c_void, c_int};
use libc::{pthread_create, pthread_join, usleep};
use std::ptr::null_mut;
use apue::PthreadExpect;
use apue::my_libc::pthread_exit;

extern "C" fn thr_fn1(_: *mut c_void) -> *mut c_void {
    unsafe {usleep(100)};
    println!("thread 1 returning");
    1 as _
}

extern "C" fn thr_fn2(_: *mut c_void) -> *mut c_void {
    println!("thread 2 returning");
    unsafe {pthread_exit(2 as _)};
    99 as _
}

fn main() {
    unsafe {
        let (mut tid1, mut tid2, mut tret) = std::mem::uninitialized();
        pthread_create(&mut tid1, null_mut(), thr_fn1, null_mut()).expect("can't create thread 1");
        pthread_create(&mut tid2, null_mut(), thr_fn2, null_mut()).expect("can't create thread 2");

        pthread_join(tid1, &mut tret).expect("can't join with thread 1");
        println!("thread 1 exit code: {}", tret as c_int);
        pthread_join(tid2, &mut tret).expect("can't join with thread 2");
        println!("thread 2 exit code: {}", tret as c_int);
    }
}