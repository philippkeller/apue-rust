/// Figure 10.7: simple, incomplete implementation of sleep

extern crate apue;
extern crate libc;

use libc::{c_int, SIGALRM, SIG_ERR};
use libc::{alarm, signal, pause};

fn sig_alrm(_: c_int) {
    // nothing to do, just return to wake up the pause
}

fn sleep1(seconds: u32) -> u32 {
    unsafe {
        if signal(SIGALRM, sig_alrm as usize) == SIG_ERR {
            return seconds;
        }
        alarm(seconds);   // start timer
        pause();          // next caught signal wakes up up
        alarm(0)          // turn off timer, return unslept time
    }
}

fn main() {
    println!("good night, diriding-ding!");
    sleep1(1);
    println!("good morning, boo!");
}