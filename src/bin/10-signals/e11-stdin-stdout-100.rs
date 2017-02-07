/// Exercise 10.11: Modify Figure 3.5 as follows:

/// (a) change BUFFSIZE to 100;
/// (b) catch the SIGXFSZ signal using the signal_intr function, printing a message
///     when it’s caught, and returning from the signal handler; and
/// (c) print the return value from write if the requested number of bytes wasn’t written.
///
/// Modify the soft RLIMIT_FSIZE resource limit (Section 7.11) to 1,024 bytes and run your
/// new program, copying a file that is larger than 1,024 bytes.
/// (Try to set the soft resource limit from your shell. If you can’t do this from your shell,
/// call setrlimit directly from the program.) Run this program on the different systems that
/// you have access to. What happens and why?
///
/// Status: compiles, did not test yet

extern crate libc;
#[macro_use(as_void)]
extern crate apue;
extern crate errno;

use libc::{SIGXFSZ, STDIN_FILENO, STDOUT_FILENO, read, write, c_int};
use apue::{LibcResult, signal_intr};
use errno::errno;
use std::io::Write;

fn exceed_filesize_limit(_:c_int) {
    println!("exceed filesize limit caught");
}

fn main() {
    let buffsize = 100;
    unsafe {
        signal_intr(SIGXFSZ, exceed_filesize_limit).to_option().expect("can't set SIGXFSZ");
        let mut num_loops = 0;
        let buf = vec![0; buffsize];
        while let Some(n) = read(STDIN_FILENO, as_void!(buf), buffsize).to_option() {
            if write(STDOUT_FILENO, as_void!(buf), n as _) != n {
                println!("write error");
            }
            num_loops += 1;
        }
        if errno().0 > 0 {
            panic!("read error");
        }
        writeln!(&mut std::io::stderr(), "total loops: {}", num_loops).unwrap();
    }
}
