/// Figure 10.5 Call a nonreentrant function from a signal handler
///
/// Findings:
///
/// - on macos most of the time the program just freezes after 2-3 prints
///   of "in signal handler"
/// - only in one run (out of about 30) there was a malloc error shown
/// - on linux it mostly freezes after only one cycle
/// - in about 10% of all cases it shows "return value corrupted! pw_name = root"
/// - when running the c code from the book it shows the same behaviour
///
/// The lock is most likely due to a malloc who is threadsafe through
/// locking which results in a deadlock. Explanation here:
/// http://stackoverflow.com/a/3941563/119861
#[macro_use(cstr)]
extern crate apue;
extern crate libc;

use libc::{c_int, SIGALRM};
use libc::{getpwnam, alarm, signal, printf};
use apue::LibcResult;
use std::ffi::CStr;

extern "C" fn my_alarm(_: c_int) {
    unsafe {
        printf(cstr!("in signal handler\n"));
        getpwnam(cstr!("root")).to_option().expect("getpwnam(root) error");
        alarm(1);
    }
}

fn main() {
    unsafe {
        signal(SIGALRM, my_alarm as usize);
        alarm(1);
        loop {
            let pwd = getpwnam(cstr!("nobody")).to_option().expect("getpwnam error");
            let pw_name = CStr::from_ptr((*pwd).pw_name).to_str().unwrap();
            if pw_name != "nobody" {
                println!("return value corrupted! pw_name = {}", pw_name);
            }
        }
    }
}
