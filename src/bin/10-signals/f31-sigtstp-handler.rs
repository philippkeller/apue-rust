/// Figure 10.31 How to handle SIGTSTP
///
/// Status: is complete and compiles, didn't try it out yet


#[macro_use(as_void)]
extern crate apue;
extern crate libc;

use std::mem::uninitialized;
use std::ptr::null_mut;
use libc::{c_int, SIGTSTP, SIG_UNBLOCK, SIG_DFL, SIG_IGN, STDIN_FILENO, STDOUT_FILENO};
use libc::{sigemptyset, sigaddset, signal, kill, getpid, read, write};
use apue::my_libc::sigprocmask;
use apue::{LibcResult, err_sys};

const BUFFSIZE:usize = 1024;

unsafe fn sig_tstp(_:c_int) {
    // move cursor to lower left corner, reset tty mode
    // unblock SIGTSTP, since it's blocked while we're readint it
    let mut mask = uninitialized();
    sigemptyset(&mut mask);
    sigaddset(&mut mask, SIGTSTP);
    sigprocmask(SIG_UNBLOCK, &mask, null_mut());

    signal(SIGTSTP, SIG_DFL); // reset disposition to default
    kill(getpid(), SIGTSTP); // and send the signal to ourself
    // we won't return from the kill until we're continued
    signal(SIGTSTP, sig_tstp as usize); // reestablish signal handler
    // ... reset tty mode, redraw screen ...
}

fn main() {
    unsafe {
        if signal(SIGTSTP, SIG_IGN) == SIG_DFL {
            signal(SIGTSTP, sig_tstp as usize);
        }
        let buf = vec![0; BUFFSIZE];
        while let Some(n) = read(STDIN_FILENO, as_void!(buf), BUFFSIZE).to_option() {
            if write(STDOUT_FILENO, as_void!(buf), n as _) != n {
                err_sys("write error");
            }
        }
    }
}