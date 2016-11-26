/// Figure 1.4 Copy standard input to standard output
///
/// $ echo asdf | f04-copy-stdin-stdout
/// asdf

extern crate libc;
#[macro_use(as_void)]
extern crate apue;

use libc::{STDIN_FILENO, STDOUT_FILENO, read, write};
use apue::LibcResult;

const BUFSIZE: usize = 4096;

fn main() {
    unsafe {
        let n = 0;
        let buf: [u8; BUFSIZE] = std::mem::uninitialized();
        while let Some(n) = read(STDIN_FILENO, as_void!(buf), BUFSIZE).to_option() {
            if write(STDOUT_FILENO, as_void!(buf), n as _) != n as _ {
                panic!("write error");
            }
        }
        if n < 0 {
            println!("read error");
        }
    }
}
