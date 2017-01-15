#![feature(drop_types_in_const)]

/// Figure 10.23 Using sigsuspend to wait for a global variable to be set
///
/// Status: after posting http://stackoverflow.com/questions/41655118/are-static-variables-volatile
/// I was sure that my solution needs an Arc<>, but then I cannot initiate a Arc<> static
/// variable as it doesn't let me do function calls. Next option: a closure which lets me
/// reference quitflag from within sig_int, but that doesn't work as the closure signature
/// is wrong.

extern crate libc;
extern crate apue;

use libc::{c_int, SIGINT, SIG_ERR, SIGQUIT, SIG_BLOCK, SIG_SETMASK};
use libc::{signal, sigemptyset, sigaddset};
use apue::my_libc::{sigprocmask, sigsuspend};
use apue::LibcResult;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::sync::Arc;

fn main() {
    unsafe {
        let (mut newmask, mut oldmask, mut zeromask) = std::mem::uninitialized();

        // closest I could find for volatile sig_atomic_t in Rust
        let mut quitflag:Arc<AtomicBool> = Arc::new(ATOMIC_BOOL_INIT);
        let sig_int = |signo| {
            match signo {
                SIGINT => println!("\ninterrupt"),
                SIGQUIT => {
                    println!("\nquit");
                    quitflag.store(true, Ordering::SeqCst);
                },
                _ => panic!("unexpected signal"),
            };
        };

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
        quitflag.store(true, Ordering::SeqCst);
        while quitflag.fetch_or(true, Ordering::Relaxed) {
            sigsuspend(&zeromask);
            println!("exit of suspended, checking bool");
        }
        sigprocmask(SIG_SETMASK, &oldmask, std::ptr::null_mut()).to_option().expect("SIG_SETMASK error");
    }
}