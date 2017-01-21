/// Figure 10.26: Using system to invoke the ed editor
///
/// Status: compiles and also invokes ed but when quitting ed
/// sig_chld is not executed..


#[macro_use(cstr)]
extern crate apue;
extern crate libc;

use apue::LibcResult;
use libc::{SIGINT, SIGCHLD, c_int};
use libc::{signal, system, printf};

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
        system(cstr!("/bin/ed")).to_option().expect("system() error");
    }
}