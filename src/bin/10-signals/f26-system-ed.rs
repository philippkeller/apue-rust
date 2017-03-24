/// Figure 10.26: Using system to invoke the ed editor
///
/// Note that we are using `system` from Figure 8.22
/// (was not clear to me first, so SIGINT and SIGQUIT
/// did not cause sig_int and sig_chld to be called)
/// Also tested with apue::system2 which behaves
/// the same way as libc::system (blocking SIGINT
/// and SIGQUIT)

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

fn sig_int(_: c_int) {
    unsafe {
        printf(cstr!("caught SIGINT\n"));
    }
}

fn sig_chld(_: c_int) {
    unsafe {
        printf(cstr!("caught SIGCHLD\n"));
    }
}

fn main() {
    unsafe {
        signal(SIGINT, sig_int as usize).check_not_sigerr().expect("signal(SIGINT) error");
        signal(SIGCHLD, sig_chld as usize).check_not_sigerr().expect("signal(SIGCHLD) error");
        system("/bin/ed").expect("system() error");
    }
}
