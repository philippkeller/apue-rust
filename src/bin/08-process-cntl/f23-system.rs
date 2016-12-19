/// Figure 8.22 The system function, without signal handling and
/// Figure 8.23 Calling the system function
///
/// $ f23-system 2>&1 | grep -v "[0-9]:" # remove all lines with times
/// normal termination, exit status = 0
/// sh: nosuchcommand: command not found
/// normal termination, exit status = 127
/// normal termination, exit status = 44

extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;

use libc::{fork, c_char, c_int, _exit, waitpid, EINTR, WSTOPSIG, WEXITSTATUS, WIFSTOPPED,
           WCOREDUMP, WTERMSIG, WIFSIGNALED, WIFEXITED};
use apue::{LibcResult, err_sys};

extern "C" {
    pub fn execl(__path: *const c_char, __arg0: *const c_char, ...) -> c_int;
}

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

unsafe fn system(cmdstring: &str) -> Option<i32> {
    if let Some(pid) = fork().to_option() {
        match pid {
            0 => {
                // child
                execl(cstr!("/bin/sh"),
                      cstr!("sh"),
                      cstr!("-c"),
                      cstr!(cmdstring),
                      0 as *const c_char);
                _exit(127);
            }
            _ => {
                // parent
                let mut status = 0;
                while waitpid(pid, &mut status, 0) < 0 {
                    if errno::errno().0 != EINTR {
                        return None;
                    }
                }
                return Some(status);
            }
        }
    } else {
        return None;
    }
}

fn main() {
    for cmd in ["date", "nosuchcommand", "who; exit 44"].into_iter() {
        if let Some(status) = unsafe { system(cmd) } {
            unsafe { pr_exit(status) };
        } else {
            err_sys(&format!("{} error", cmd));
        }
    }
}
