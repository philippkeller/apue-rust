/// Figure 3.1: Test whether standard input is capable of seeking
///
/// Takaway: yea, you cannot
///
/// $ f01-seek-stdio < /etc/passwd
/// seek OK
/// $ cat < /etc/passwd | f01-seek-stdio
/// cannot seek

extern crate libc;
extern crate apue;

use libc::{STDIN_FILENO, SEEK_CUR, lseek};
use apue::LibcResult;

fn main() {
    if unsafe { lseek(STDIN_FILENO, 0, SEEK_CUR).check_not_negative() }.is_ok() {
        println!("seek OK");
    } else {
        println!("cannot seek");
    }
}
