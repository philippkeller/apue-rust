/// Figure 10.15 Example of signal sets and sigprocmask
///
/// Finding: on OSX signals seem to be not queued as well. Running f15-sigpending and then
/// immediately firing ^\ results in only one echo of "caught SIGQUIT"
///
/// test.py doesn't handle the following lines correctly (yet), -> only in // and not in ///:
// $ f15-sigpending &
// $ pkill -SIGQUIT f15-sigpending && sleep 2 && pkill -SIGQUIT f15-sigpending
// SIGQUIT pending
// caught SIGQUIT
// SIGQUIT unblocked

extern crate libc;
extern crate apue;

use libc::{c_int, SIG_SETMASK, SIG_BLOCK, SIGQUIT, SIG_ERR, SIG_DFL};
use libc::{sigemptyset, sigaddset, sigismember, sleep};
use apue::my_libc::{sigprocmask, sigpending};
use apue::{LibcResult, signal};

fn sig_quit(_: c_int) {
    println!("caught SIGQUIT");
    if unsafe { libc::signal(SIGQUIT, SIG_DFL) } == SIG_ERR {
        panic!("can't reset SIGQUIT");
    }
}

fn main() {
    unsafe {
        if signal(SIGQUIT, sig_quit) == SIG_ERR {
            panic!("can't catch SIGQUIT");
        }
        // Block SIGQUIT and save current signal mask.
        let (mut newmask, mut oldmask, mut pendmask) = std::mem::uninitialized();
        sigemptyset(&mut newmask);
        sigaddset(&mut newmask, SIGQUIT);
        sigprocmask(SIG_BLOCK, &newmask, &mut oldmask).check_not_negative().expect("SIG_BLOCK error");
        sleep(2); // time to send SIGQUIT -> will remain pending
        sigpending(&mut pendmask).check_not_negative().expect("sigpending error");
        if sigismember(&pendmask, SIGQUIT) == 1 {
            println!("SIGQUIT pending");
        }

        // restore signal mask which unblocks SIGQUIT
        sigprocmask(SIG_SETMASK, &oldmask, std::ptr::null_mut())
            .check_not_negative()
            .expect("SIG_SETMASK error");
        println!("SIGQUIT unblocked");
        sleep(2); // <-- SIGQUIT will hit here with core dump
    }
}
