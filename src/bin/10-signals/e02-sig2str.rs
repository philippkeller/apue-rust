/// Exercise 10.2: Implement the sig2str function described in Section 10.22.
///
/// $ e02-sig2str
/// Signal: Interrupt

extern crate apue;
extern crate libc;

use libc::{SIGINT, c_char};

extern {
    static sys_siglist: [*const c_char; 65usize];
}

fn sig2str(signo:i32) -> &'static str {
    unsafe {
        std::ffi::CStr::from_ptr(sys_siglist[signo as usize]).to_str().expect("invalid utf8 string")
    }
}

fn main() {
    println!("Signal: {}", sig2str(SIGINT));
}