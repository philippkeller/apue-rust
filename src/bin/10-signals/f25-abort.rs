/// Figure 10.25 Implementation of POSIX.1 abort
///
/// Status: compiles, not tested yet (assuming this function will be
/// used in the future in the book)

extern crate libc;
extern crate apue;

use libc::{SIG_DFL, SIGABRT, SIG_IGN, SIG_SETMASK};
use libc::{sigaction, fflush, sigdelset, sigfillset, getpid, kill, exit};
use std::ptr::{null, null_mut};
use apue::my_libc::sigprocmask;


unsafe fn abort() {
    let mut action: sigaction = std::mem::uninitialized();
    // load current signal handler and flags into action
    sigaction(SIGABRT, null(), &mut action);
    if action.sa_sigaction == SIG_IGN {
        // Caller can’t ignore SIGABRT, if so reset to default
        action.sa_sigaction = SIG_DFL;
        sigaction(SIGABRT, &action, null_mut());
    }
    if action.sa_sigaction == SIG_DFL {
        fflush(null_mut());
    }
    let mut mask = std::mem::uninitialized();
    sigfillset(&mut mask); // block all signals
    sigdelset(&mut mask, SIGABRT); // leave only SIGABRT through
    sigprocmask(SIG_SETMASK, &mask, null_mut());
    kill(getpid(), SIGABRT); // say bye bye to mummy

    // if we're here, process caught SIGABRT and returned
    fflush(null_mut());
    action.sa_sigaction = SIG_DFL;
    sigaction(SIGABRT, &action, null_mut());
    sigprocmask(SIG_SETMASK, &mask, null_mut()); // what, again? Really?
    kill(getpid(), SIGABRT);
    exit(1); // now THIS should now never be executed
}

fn main() {
    let _ = abort;
}
