/// Type in the program that copies a file using line-at-a-time I/O 
/// (fgets and fputs) from Figure 5.5, but use a MAXLINE of 4.

extern crate libc;
extern crate apue;

use libc::{fopen, fgets, fputs, printf};
use apue::*;

const BUFLEN: usize = 4;

fn main() {
	unsafe {
		let mut args = std::env::args();
		if args.len() != 3 {
			println!("usage:\n{} /path/to/input /path/to/output", args.next().unwrap());
			std::process::exit(1);
		}
		args.next(); // skip filename
		let f_in = args.next().unwrap();
		let f_out = args.next().unwrap();
		let fd_in = fopen(f_in.to_ptr(), "r".to_ptr());
		let fd_out = fopen(f_out.to_ptr(), "w".to_ptr());

		let buffer:[u8;BUFLEN] = std::mem::uninitialized();
		while !fgets(buffer.as_ptr() as *mut i8, BUFLEN as i32, fd_in).is_null() {
			printf("buffer = %s\n".to_ptr(), buffer.as_ptr());
			fputs(buffer.as_ptr() as *mut i8, fd_out);
		}
	}
}

// # Solution:

// What happens if you copy lines that exceed this length? 
// Explain what is happening.

// Answer: the line is cut into chunks of 3 (3 bytes + null)
// the last chunk of a line can be smaller, so e.g. just 1 byte + null
//
// e.g.
// hansaplasti
// - han
// - sap
// - las
// - ti