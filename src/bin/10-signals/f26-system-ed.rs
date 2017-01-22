/// Figure 10.26: Using system to invoke the ed editor

/// Proof that it works:

// $ f26-system-ed
// a
// Here is one line of text
// .
// 1,$p
// Here is one line of text
// w temp.foo
// 25
// q
// caught SIGCHLD
//
// ~/oss/apue on  master! ⌚ 21:56:22
// $ f26-system-ed
// a
// hello, world
// .
// 1,$p
// hello, world
// w temp.foo
// 13
// ^C
// ?
// caught SIGINT
// q
// caught SIGCHLD


#[macro_use(cstr)]
extern crate apue;
extern crate libc;

use apue::{LibcResult, system};
use libc::{SIGINT, SIGCHLD, c_int};
use libc::{signal, printf};

fn sig_int(_:c_int) {
    unsafe {
        printf(cstr!("caught SIGINT\n"));
    }
}

fn sig_chld(_:c_int) {
    unsafe {
        printf(cstr!("caught SIGCHLD\n"));
    }
}

fn main() {
    unsafe {
        signal(SIGINT, sig_int as usize).to_option().expect("signal(SIGINT) error");
        signal(SIGCHLD, sig_chld as usize).to_option().expect("signal(SIGCHLD) error");
        system("/bin/ed").expect("system() error");
    }
}