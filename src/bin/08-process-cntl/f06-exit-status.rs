/// Figure 8.5: Print a description of the exit status, and
/// Figure 8.6: Demonstrate various exit statuses
///
/// Takeaway: Rust intercepts the division by zero with its panic handler.
/// To make the exception "fall through" we need to replace rusts panic handler
/// by our own (at least that's what the guys on IRC suggested). Sadly in order to
/// go sure that it was *really* a division by zero we need to do a string comparison..
///
/// $ f06-exit-status
/// normal termination, exit status = 7
/// abnormal termination, signal number = 6
/// abnormal termination, signal number = 8

extern crate libc;
extern crate apue;

use libc::{c_int, WIFEXITED, WEXITSTATUS, WIFSIGNALED, WTERMSIG, WCOREDUMP, WIFSTOPPED, WSTOPSIG, SIGFPE};
use libc::{exit, wait, fork, abort, raise};
use apue::{LibcResult, err_sys};
use std::panic;

unsafe fn pr_exit(status: c_int) {
    if WIFEXITED(status) {
        println!("normal termination, exit status = {}", WEXITSTATUS(status));
    } else if WIFSIGNALED(status) {
        println!("abnormal termination, signal number = {} {}",
                 WTERMSIG(status),
                 if WCOREDUMP(status) {
                     " (core file generated)"
                 } else {
                     ""
                 });
    } else if WIFSTOPPED(status) {
        println!("child stopped, signal number = {}", WSTOPSIG(status));
    }
}

fn handle_panic(e: &panic::PanicInfo) {
    match e.payload().downcast_ref::<String>() {
        Some(as_string) if as_string == "attempt to divide by zero" => {
            unsafe { raise(SIGFPE) };
        }
        _ => {
            // unknown error
            panic!("unknown error occurred");
        },
    }
}

fn main() {
    panic::set_hook(Box::new(handle_panic));
    unsafe {
        let mut status: c_int = 0;
        let mut pid = fork().to_option().expect("fork error");
        if pid == 0 {
            // child
            exit(7);
        }
        if wait(&mut status) != pid {
            // wait for child
            err_sys("wait error");
        }
        pr_exit(status);

        pid = fork().to_option().expect("fork error");
        if pid == 0 {
            // child
            abort(); // generate SIGABRT
        }
        if wait(&mut status) != pid {
            // wait for child
            err_sys("wait error");
        }
        pr_exit(status);

        pid = fork().to_option().expect("fork error");
        if pid == 0 {
            // child
            status /= 0; // divide by 0 generates SIGFPE
        }

        if wait(&mut status) != pid {
            err_sys("wait error");
        }
        pr_exit(status);
    }
}