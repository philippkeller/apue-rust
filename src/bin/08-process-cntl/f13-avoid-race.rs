extern crate libc;

use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use libc::{fork, setbuf, c_char, c_int, STDOUT_FILENO, fdopen, usleep, FILE};

const N: usize = 10;

extern "C" {
    pub fn putc(arg1: c_int, arg2: *mut FILE) -> c_int;
}

fn charatatime(out: *mut FILE, s: &str) {
    for c in s.chars() {
        unsafe {
            putc(c as i32, out);
            usleep(20);
        }
    }
}

fn main() {
    let stdout = unsafe {
        let stdout = fdopen(STDOUT_FILENO, &('w' as c_char));
        setbuf(stdout, std::ptr::null_mut());
        stdout
    };
    let data = Arc::new(Mutex::new(0));

    match unsafe { fork() } {
        0 => {
            data.lock().unwrap();
            charatatime(stdout, "child is writing...")
        },
        _ => {
            data.lock().unwrap();
            charatatime(stdout, "parent is writing...")
        }
    }
}