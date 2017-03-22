/// Figure 1.5 Copy standard input to standard output, using standard I/O
///
/// $ echo asdf | f05-copy-stdin-stdout2
/// asdf

extern crate libc;
extern crate apue;

use libc::ferror;
use apue::LibcResult;
use apue::my_libc::{putc, getc, stdin, stdout};


fn main() {
    unsafe {
        while let Ok(c) = getc(stdin).check_not_negative() {
            assert!(putc(c, stdout) >= 0, "output error");
        }
        assert!(ferror(stdin) == 0, "input error");
    }
}
