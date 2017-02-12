/// Exercise 10.12: Write a program that calls fwrite with a large buffer
/// (about one gigabyte). Before calling fwrite, call alarm to schedule a
/// signal in 1 second. In your signal handler, print that the signal was
/// caught and return. Does the call to fwrite complete? What’s happening?
///
/// Findings:
/// - both on OSX and Linux the alarm is immediately triggered
/// - alarm doesn't stop the fwrite call, all 1GB data is written on disk still
/// - initially wanted to create the buffer with
///   `let buffer: [i8; SIZE] = [50; SIZE]` which resulted in a segfault.
///   After some debugging with strace I found out that it was failing because
///   the memory is allocated in the stack and `getrlimit(RLIMIT_STACK)` returned
///   a max stack size of 2^23 bytes -> the max size of a buffer in the stack
///   is 2^22 because some bytes are needed for the code and the other vars.

#[macro_use(as_void, cstr)]
extern crate apue;
extern crate libc;
extern crate errno;

use libc::{c_int, SIGALRM, fwrite, fopen, alarm};
use apue::{LibcResult, signal};

fn sig_alrm(_:c_int) {
    println!("alarm has happened..");
}

const SIZE:usize = 1 << 32;
fn main() {
    unsafe {
        signal(SIGALRM, sig_alrm).to_option().expect("couldn't set alarm");
        alarm(1);
        let buffer:Vec<u8> = Vec::with_capacity(SIZE);
        let file = fopen(cstr!("/tmp/10_fwrite_1gb.txt"), cstr!("w")).to_option().expect("can't open file");
        fwrite(as_void!(buffer), 1, SIZE, file).to_option().expect("can't write to file");
    }
}