/// Figure 3.5: Copy standard input to standard output

extern crate libc;
#[macro_use(as_void)]
extern crate apue;
extern crate errno;

use libc::{STDIN_FILENO, STDOUT_FILENO, read, write};
use apue::LibcResult;
use errno::errno;

const BUFFSIZE:usize = 4096;

fn main() {
    unsafe {
        let buf:[u8;BUFFSIZE] = std::mem::uninitialized();
        println!("enter some words, to stop just hit return on empty line");
        while let Some(n) = read(STDIN_FILENO, as_void!(buf), BUFFSIZE).to_option() {
            if n == 1 && buf[0] == '\n' as _ {
                break
            }
            assert!(write(STDOUT_FILENO, as_void!(buf), n as _) == n, "write error");
        }
        if errno().0 > 0 {
            panic!("read error");
        }
    }
}