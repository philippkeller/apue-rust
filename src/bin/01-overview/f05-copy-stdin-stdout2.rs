extern crate libc;
extern crate apue;

use libc::{c_int, c_char, FILE, STDIN_FILENO, STDOUT_FILENO, fdopen, ferror};
use apue::LibcResult;

extern "C" {
    pub fn putc(arg1:c_int, arg2: *mut FILE) -> c_int;
    pub fn getc(arg1: *mut FILE) -> c_int;
}

fn main() {
	unsafe {
	    let stdin = fdopen(STDIN_FILENO, &('r' as c_char));
	    let stdout = fdopen(STDOUT_FILENO, &('w' as c_char));
	    while let Some(c) = getc(stdin).to_option() {
	    	assert!(putc(c, stdout) >= 0, "output error");
	    }
	    assert!(ferror(stdin) == 0, "input error");
	}
}