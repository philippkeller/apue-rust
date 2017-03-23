/// Figure 1.4 Copy standard input to standard output
///
/// $ echo asdf | f04-copy-stdin-stdout
/// asdf

extern crate libc;
#[macro_use(as_void)]
extern crate apue;
extern crate errno;

use libc::{STDIN_FILENO, STDOUT_FILENO, read, write};
use apue::LibcResult;

const BUFSIZE: usize = 4096;

fn main() {
    unsafe {
        let buf: [u8; BUFSIZE] = std::mem::uninitialized();
        while let Ok(n) = read(STDIN_FILENO, as_void!(buf), BUFSIZE).check_positive() {
            if write(STDOUT_FILENO, as_void!(buf), n as _) != n as _ {
                panic!("write error");
            }
        }
        if errno::errno().0 != 0 {
            println!("read error");
        }
    }
}
