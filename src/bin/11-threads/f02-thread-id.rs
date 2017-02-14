/// Figure 11.2 Printing thread IDs
///
/// $ f02-thread-id | sed 's/[0-9]//g; s/x[a-f]*//g'
/// main thread: pid  tid  ()
/// new thread: pid  tid  ()

extern crate libc;
extern crate apue;

use libc::{c_void, pthread_t};
use libc::{pthread_create, getpid, pthread_self, usleep};
use std::ptr::null_mut;
use apue::PthreadExpect;

fn printids(s: &str) {
    unsafe {
        let pid = getpid();
        let tid = pthread_self();
        println!("{} pid {} tid {} (0x{:x})", s, pid, tid, tid);
    }
}

extern "C" fn thr_fn(_: *mut c_void) -> *mut c_void {
    printids("new thread:");
    0 as _
}

fn main() {
    unsafe {
        let mut ntid: pthread_t = std::mem::zeroed();
        pthread_create(&mut ntid, null_mut(), thr_fn as _, null_mut())
            .expect("can't create thread");
        printids("main thread:");
        usleep(100);
    }
}
