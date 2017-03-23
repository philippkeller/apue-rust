/// Figure 10.22 Protecting a critical region from a signal

extern crate libc;
extern crate apue;
extern crate errno;

use libc::{c_int, SIGINT, SIG_ERR, SIGUSR1, SIG_BLOCK, SIG_SETMASK};
use libc::{signal, sigemptyset, sigaddset};
use apue::my_libc::{sigprocmask, sigsuspend};
use apue::{pr_mask, LibcResult};
use std::mem::uninitialized as uninit;

fn sig_int(_: c_int) {
    pr_mask("\nin sig_int: ");
}

fn main() {
    unsafe {
        let (mut waitmask, mut newmask, mut oldmask) = (uninit(), uninit(), uninit());
        pr_mask("program start: ");
        if signal(SIGINT, sig_int as usize) == SIG_ERR {
            println!("signal(SIGINT) error");
        }
        sigemptyset(&mut waitmask);
        sigaddset(&mut waitmask, SIGUSR1);
        sigemptyset(&mut newmask);
        sigaddset(&mut newmask, SIGINT);

        // Block SIGINT and save current signal mask
        sigprocmask(SIG_BLOCK, &newmask, &mut oldmask)
            .check_not_negative()
            .expect("SIG_BLOCK error");
        // Critical region of code
        pr_mask("in critical region: ");
        // Pause, allowing all signals except SIGUSR1
        sigsuspend(&waitmask).check_minus_one().expect("sigsuspend error");
        pr_mask("after return from sigsuspend: ");
        // reset signal mask which unblocks SIGINT
        sigprocmask(SIG_SETMASK, &oldmask, std::ptr::null_mut())
            .check_not_negative()
            .expect("SIG_SETMASK error");
        pr_mask("program exit: ");
    }
}
