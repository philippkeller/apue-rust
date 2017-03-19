/// Figure 8.12 Program with a race condition
///
/// Takeaway: on OSX it needed at least usleep(20) in order
/// to experience the race condition
///
/// $ f12-race-condition | awk 'END{print NR}'
/// 2

extern crate libc;
extern crate apue;

use libc::{c_char, FILE, STDOUT_FILENO, fork, setbuf, fdopen, usleep};
use apue::LibcResult;
use apue::my_libc::putc;


unsafe fn charatatime(out: *mut FILE, s: &str) {
    for c in s.chars() {
        putc(c as i32, out);
        usleep(20);
    }
}

fn main() {
    unsafe {
        // set unbuffered
        let stdout = fdopen(STDOUT_FILENO, &('w' as c_char));
        setbuf(stdout, std::ptr::null_mut());
        let pid = fork().to_option().expect("fork error");
        match pid {
            0 => charatatime(stdout, "output from child \n"),
            _ => charatatime(stdout, "output from parent \n"),
        }
    }
}
