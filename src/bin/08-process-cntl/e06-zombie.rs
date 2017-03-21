/// Exercise 8.6: Write a program that creates a zombie, and then call system
/// to execute the ps(1) command to verify that the process is a zombie.
///
/// Takeaway: does only work on Linux, on MacOs the child process is somehow
/// reaped automatically, at least ps doesn't show it
/// More details here: http://stackoverflow.com/questions/41427982

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{fork, sleep, exit, system};
use apue::LibcResult;

fn main() {
    unsafe {
        let pid = fork().check_not_negative().expect("fork error");
        if pid == 0 {
            exit(0);
        }
        sleep(99);
        system(cstr!("ps -fo pid,ppid,state,tty,command"));
    }
}
