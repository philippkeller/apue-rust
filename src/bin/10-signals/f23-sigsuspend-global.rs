/// Figure 10.23 Using sigsuspend to wait for a global variable to be set
///
/// The thing that needed the most work is setting the quitflag in the signal
/// so that the main loop is seeing the change, plus telling the compiler not to
/// "compile away" the while loop as apparently there is no code setting the flag to false.
///
/// The code I ended up with (static AtomicBool) instance was what I came up very early
/// (before asking on Stackoverflow), but I mistakenly set QUITFLAG to true in the signal
/// which made the while loop to never exit. This sent me to a journey in failed attempts:
///
/// - after posting http://stackoverflow.com/questions/41655118
///   I was sure that my solution needs an Arc<>, but then I cannot initiate a Arc<> static
///   variable as it doesn't let me do function calls
/// - turn the signalling funciton into a closure and put the quitflag variable inside main.
///   This won't ever work because closures are inherently different than functions as they
///   capture their environment, see http://stackoverflow.com/questions/32270030
///
// $ f23-sigsuspend-global
// $ pkill -SIGINT f23-sugsuspend-global
// interrupt
// $ pkill -SIGQUIT f23-sugsuspend-global
// quit
// [1]  + 92089 done       f23-sugsuspend-global


extern crate libc;
extern crate apue;

use libc::{c_int, SIGINT, SIG_ERR, SIGQUIT, SIG_BLOCK, SIG_SETMASK};
use libc::{signal, sigemptyset, sigaddset};
use apue::my_libc::{sigprocmask, sigsuspend};
use apue::LibcResult;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

// closest I could find for volatile sig_atomic_t in Rust
static mut QUITFLAG: AtomicBool = ATOMIC_BOOL_INIT;

fn sig_int(signo: c_int) {
    unsafe {
        match signo {
            SIGINT => println!("interrupt"),
            SIGQUIT => {
                println!("quit");
                QUITFLAG.store(false, Ordering::SeqCst);
            }
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
        QUITFLAG.store(true, Ordering::SeqCst);
        while QUITFLAG.fetch_or(true, Ordering::Relaxed) {
            sigsuspend(&zeromask);
        }
        sigprocmask(SIG_SETMASK, &oldmask, std::ptr::null_mut())
            .to_option()
            .expect("SIG_SETMASK error");
    }
}
