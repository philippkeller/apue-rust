/// Figure 10.2 Simple program to catch SIGUSR1 and SIGUSR2
///
/// Example session:
/// (doesn't work with test.py, that's why only put in // comments
///
// $ f02-sigusr &
// $ kill -USR1 $!
// received SIGUSR1
// $ kill -USR1 $!
// received SIGUSR2

extern crate libc;
extern crate apue;

use libc::{c_int, SIGUSR1, SIGUSR2, SIG_ERR};
use libc::{signal, pause};

extern "C" fn sig_usr(signo: c_int) {
    match signo {
        SIGUSR1 => println!("received SIGUSR1"),
        SIGUSR2 => println!("received SIGUSR2"),
        _ => {
            panic!(format!("received signal {}", signo));
        }
    }
}

fn main() {
    unsafe {
        if signal(SIGUSR1, sig_usr as usize) == SIG_ERR {
            panic!("can't catch SIGUSR1");
        }
        if signal(SIGUSR2, sig_usr as usize) == SIG_ERR {
            panic!("can't catch SIGUSR2");
        }
        loop {
            pause();
        }
    }
}
