/// Figure 3.1: Test whether standard input is capable of seeking
///
/// Takaway: yea, you cannot

extern crate libc;
extern crate apue;

use libc::{STDIN_FILENO, SEEK_CUR, lseek};
use apue::LibcResult;

fn main() {
	if let Some(_) = unsafe { lseek(STDIN_FILENO, 0, SEEK_CUR).to_option() } {
		println!("seek OK");
	} else {
		println!("cannot seek");
	}
}