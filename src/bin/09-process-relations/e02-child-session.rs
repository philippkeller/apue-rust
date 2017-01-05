/// Exercise 9.2: Write a small program that calls fork and
/// has the child create a new session. Verify that the
/// child becomes a process group leader and that the child
/// no longer has a controlling terminal.
///
/// $ e02-child-session
/// that was easy..

extern crate libc;
extern crate apue;

use libc::{STDIN_FILENO, fork, getpid, setsid, tcgetpgrp};
use apue::{LibcResult};

fn main() {
    unsafe {
        let pid = fork().to_option().expect("fork error");
        if pid == 0 {
            // child: create new session
            let sid = setsid();
            // verify that I am the new process group leader
            assert!(sid == getpid());
            // verify that we
            assert!(tcgetpgrp(STDIN_FILENO) == -1);
            println!("that was easy..");
        }
    }
}