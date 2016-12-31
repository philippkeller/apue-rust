/// Figure 2.28: Program to generate accounting data
#[macro_use(cstr)]
extern crate apue;
extern crate libc;

use libc::{sleep, fork, exit, abort, c_char, kill, getpid, SIGKILL};
use apue::LibcResult;
use apue::my_libc::execl;

unsafe fn _fork() -> i32 {
    fork().to_option().expect("fork error")
}

fn main() {
    unsafe {
        // parent
        if _fork() > 0 {
            sleep(2);
            exit(2);
        }
        // 1st child
        if _fork() > 0 {
            // first child
            sleep(4);
            abort(); // terminate with core dump
        }
        // 2nd child
        if _fork() > 0 {
            execl(cstr!("/bin/dd"),
                  cstr!("dd"),
                  cstr!("if=/etc/passwd"),
                  cstr!("of=/dev/stdout"),
                  0 as *const c_char)
                .to_option()
                .expect("execl error");
            exit(7);
        }
        // 3rd child
        if _fork() > 0 {
            sleep(8);
            exit(0);
        }
        // 4th child
        sleep(6);
        kill(getpid(), SIGKILL);
        exit(6);
    }
}
