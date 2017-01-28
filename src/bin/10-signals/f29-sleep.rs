/// Figure 10.29 Reliable implementation of sleep
///
/// I must say that this signal thing gets a bit
/// boring...

extern crate apue;
extern crate libc;

use std::mem::uninitialized;
use libc::{c_int, sigaction, SIGALRM, SIGINT, SIG_SETMASK, SIG_BLOCK};
use libc::{sigemptyset, sigaddset, sigdelset, alarm};
use apue::my_libc::{sigprocmask, sigsuspend};

fn sig_alrm(_:c_int) {
    // nothing to do, just returning wakes up sigsuspend()
}

unsafe fn sleep(seconds: u32) -> u32 {
    let mut newact:sigaction = uninitialized();

    // set our handler, save previous information
    newact.sa_sigaction = sig_alrm as usize;
    sigemptyset(&mut newact.sa_mask);
    newact.sa_flags = 0;
    let mut oldact = uninitialized();
    sigaction(SIGALRM, &newact, &mut oldact);

    // block SIGALRM and save current signal mask
    let (mut newmask, mut oldmask) = uninitialized();
    sigemptyset(&mut newmask);
    sigaddset(&mut newmask, SIGALRM);
    sigprocmask(SIG_BLOCK, &newmask, &mut oldmask);

    alarm(seconds);
    let mut suspmask = oldmask;

    // make sure SIGALRM isn't blocked
    sigdelset(&mut suspmask, SIGALRM);

    // wait for any signal to be caught
    sigsuspend(&suspmask);

    // some signal has been caught, SIGALRM is now blocked
    let unslept = alarm(0);

    // reset previous action
    sigaction(SIGALRM, &oldact, std::ptr::null_mut());

    // reset signal mask, which unblocks SIGALRM
    sigprocmask(SIG_SETMASK, &oldmask, std::ptr::null_mut());
    unslept
}

fn main() {
    unsafe {
        println!("{}", sleep(10));
    };
}