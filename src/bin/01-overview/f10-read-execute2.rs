/// Figure 1.10 Read commands from standard input and execute them
/// source from Figure 1.7 and added the signal handling
///
/// $ echo tty | f10-read-execute2 2>&1
/// not a tty
/// % %


extern crate libc;
#[macro_use(cstr, as_char)]
extern crate apue;

use libc::{STDIN_FILENO, SIGINT, SIG_ERR, c_char, c_int, printf, strlen, fgets, fdopen, fork,
           waitpid, signal};
use apue::{array_to_string, LibcResult};
use apue::my_libc::execlp;

extern "C" fn sig_int(_: c_int) {
    unsafe {
        printf(cstr!("interrupted..\n"));
        printf(cstr!("%% "));
    }
}

const MAXLINE: usize = 100;

fn main() {
    unsafe {
        let mut buf: [c_char; MAXLINE] = std::mem::uninitialized();
        let stdin = fdopen(STDIN_FILENO, &('r' as c_char));
        let mut status = 0;
        let s = sig_int;
        if signal(SIGINT, s as usize) == SIG_ERR {
            panic!("signal error");
        }
        printf(cstr!("%% ")); // print prompt (printf requires %% to print %)
        while !fgets(as_char!(buf), MAXLINE as _, stdin).is_null() {
            let len = strlen(as_char!(buf));
            if buf[len - 1] == '\n' as _ {
                buf[len - 1] = 0;
            }
            if let Ok(pid) = fork().check_not_negative() {
                if pid == 0 {
                    // child
                    execlp(as_char!(buf), as_char!(buf), 0);
                    panic!("could not execute {}", array_to_string(&buf));
                } else {
                    // parent
                    if waitpid(pid, &mut status, 0).check_not_negative().is_ok() {
                        printf(cstr!("%% "));
                    } else {
                        panic!("waitpid error");
                    }
                }
            } else {
                panic!("fork error");
            }
        }
    }
}
