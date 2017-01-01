/// Exercise 8.6: Write a program that creates a zombie, and then call system
/// to execute the ps(1) command to verify that the process is a zombie.
///
/// Takeaway: does only work on Linux, on MacOs the child process is somehow
/// reaped automatically, at least ps doesn't show it

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{fork, sleep, exit, system};
use apue::LibcResult;

fn main() {
    unsafe {
        let pid = fork().to_option().expect("fork error");
        if pid == 0 {
            exit(0);
        }
        sleep(1);
        system(cstr!("ps -o pid,ppid,state,tty,command"));
    }
}