#![allow(dead_code)]

/// Figure 10.24 Routines to allow a parent and child to synchronize
///
/// Status: only compiles, did not yet run it to check for correctness
/// has still bugs for sure..

extern crate apue;
extern crate libc;

use apue::LibcResult;
use apue::my_libc::{sigprocmask, sigsuspend};
use libc::{SIGUSR1, SIGUSR2, SIG_BLOCK, SIG_SETMASK, c_int, pid_t, sigset_t};
use libc::{signal, sigemptyset, sigaddset, kill};
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

static mut SIGFLAG: AtomicBool = ATOMIC_BOOL_INIT;

fn sig_usr(_: c_int) {
    unsafe {
        SIGFLAG.store(true, Ordering::SeqCst);
    }
}

fn tell_wait() {
    unsafe {
        let (mut newmask, mut oldmask, mut zeromask) = std::mem::uninitialized();
        signal(SIGUSR1, sig_usr as usize).to_option().expect("signal(SIGUSR1) error");
        signal(SIGUSR2, sig_usr as usize).to_option().expect("signal(SIGUSR2) error");
        sigemptyset(&mut zeromask);
        sigemptyset(&mut newmask);
        sigaddset(&mut newmask, SIGUSR1);
        sigaddset(&mut newmask, SIGUSR2);

        // Bloock SIGUSR1 and SIGUSR2 and save current signal mask
        sigprocmask(SIG_BLOCK, &newmask, &mut oldmask).to_option().expect("SIG_BLOCK error");
    }
}

unsafe fn tell_parent(pid: pid_t) {
    kill(pid, SIGUSR2); // tell parent we're done
}

unsafe fn wait_parent(zeromask: sigset_t) {
    // run until sigflag becomes true, then set it to false again immediately
    while !SIGFLAG.fetch_xor(false, Ordering::SeqCst) {
        sigsuspend(&zeromask);
    }
}

unsafe fn tell_child(pid: pid_t) {
    kill(pid, SIGUSR1);
}

unsafe fn wait_child(zeromask: sigset_t, oldmask: sigset_t) {
    // run until sigflag becomes true, then set it to false again immediately
    while !SIGFLAG.fetch_xor(false, Ordering::SeqCst) {
        sigsuspend(&zeromask);
    }
    // Reset signal mask to original value
    sigprocmask(SIG_SETMASK, &oldmask, std::ptr::null_mut())
        .to_option()
        .expect("SIG_SETMASK error");
}

fn main() {}
