/// Figure 1.5 Copy standard input to standard output, using standard I/O
///
/// $ echo asdf | f05-copy-stdin-stdout2
/// asdf

extern crate libc;
extern crate apue;

use libc::{c_char, STDIN_FILENO, STDOUT_FILENO, fdopen, ferror};
use apue::LibcResult;
use apue::my_libc::{putc, getc};


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
