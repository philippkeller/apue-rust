/// Figure 8.22 The system function, without signal handling and
/// Figure 8.23 Calling the system function
///
/// mac only:
/// $ f23-system 2>&1 | grep -Ev "[0-9]{2}:" # remove all lines with times
/// normal termination, exit status = 0
/// sh: nosuchcommand: command not found
/// normal termination, exit status = 127
/// normal termination, exit status = 44
///
/// linux only:
/// $ f23-system 2>&1 | grep -Ev "[0-9]{2}:" # remove all lines with times
/// normal termination, exit status = 0
/// sh: 1: nosuchcommand: not found
/// normal termination, exit status = 127
/// normal termination, exit status = 44


extern crate libc;
extern crate apue;
extern crate errno;

use apue::{err_sys, pr_exit, system};


fn main() {
    for cmd in ["date", "nosuchcommand", "who; exit 44"].into_iter() {
        if let Some(status) = unsafe { system(cmd) } {
            pr_exit(status);
        } else {
            err_sys(&format!("{} error", cmd));
        }
    }
}
