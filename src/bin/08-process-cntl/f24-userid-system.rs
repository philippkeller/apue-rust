/// Figure 8.24: Execute the command-line argument using system
///
/// Takeaway: the security hole as described in the book does
/// not apply for Mac, there: the forked process has the effective
/// userid reset
///
/// $ f24-userid-system "echo hans"
/// hans
/// normal termination, exit status = 0

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{system, exit};
use std::env;
use apue::{LibcResult, pr_exit};

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        println!("command-line argument required");
        unsafe { exit(1) };
    }
    args.next(); // skip exe-name
    let status = unsafe { system(cstr!(args.next().unwrap())).to_option().expect("system() error") };
    pr_exit(status);
}