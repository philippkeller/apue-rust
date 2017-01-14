/// Figure 10.23 Using sigsuspend to wait for a global variable to be set
///
/// Status: everything work except the most interesting part which is
/// setting the static variable in sig_int and checking it in main..

extern crate libc;
extern crate apue;

use libc::{c_int, SIGINT, SIG_ERR, SIGQUIT, SIG_BLOCK, SIG_SETMASK};
use libc::{signal, sigemptyset, sigaddset};
use apue::my_libc::{sigprocmask, sigsuspend};
use apue::LibcResult;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

// closest I could find for volatile sig_atomic_t in Rust
static mut QUITFLAG:AtomicBool = ATOMIC_BOOL_INIT;

fn sig_int(signo:c_int) {
    unsafe {
        match signo {
            SIGINT => println!("\ninterrupt"),
            SIGQUIT => {
                println!("\nquit");
                QUITFLAG = AtomicBool::new(true);
            },
            _ => panic!("unexpected signal"),
        }
    }
}

fn main() {
    unsafe {
        let (mut newmask, mut oldmask, mut zeromask) = std::mem::uninitialized();
        if signal(SIGINT, sig_int as usize) == SIG_ERR {
            panic!("signal(SIGINT) error");
        }
        if signal(SIGQUIT, sig_int as usize) == SIG_ERR {
            panic!("signal(SIGQUIT) error");
        }
        sigemptyset(&mut zeromask);
        sigemptyset(&mut newmask);
        sigaddset(&mut newmask, SIGQUIT);
        // Block SIGQUIT and save current signal mask
        sigprocmask(SIG_BLOCK, &newmask, &mut oldmask).to_option().expect("SIG_BLOCK error");

        // here comes the AtomicBool into play which goes sure that whenever
        // QUITFLAG becomes true it is immediately set to false again
        // so no other Thread could "catch" it
        QUITFLAG = AtomicBool::new(true);
        while QUITFLAG.fetch_or(true, Ordering::Relaxed) {
            sigsuspend(&zeromask);
            println!("exit of suspended, checking bool");
        }
        QUITFLAG = AtomicBool::new(false);
        sigprocmask(SIG_SETMASK, &oldmask, std::ptr::null_mut()).to_option().expect("SIG_SETMASK error");
    }
}