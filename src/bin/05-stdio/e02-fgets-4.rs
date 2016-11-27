/// Exercise 5.2:
///
/// Type in the program that copies a file using line-at-a-time I/O (fgets and fputs)
/// from Figure 5.5, but use a MAXLINE of 4. What happens if you copy lines that exceed
/// this length? Explain what is happening.
///
/// $ echo hansaplasti > /tmp/e02.txt
/// $ e02-fgets-4 /tmp/e02.txt /dev/stderr 2>/dev/null
/// buffer = han
/// buffer = sap
/// buffer = las
/// buffer = ti
/// $ rm /tmp/e02.txt

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{fopen, fgets, fputs, printf};

const BUFLEN: usize = 4;

fn main() {
    unsafe {
        let mut args = std::env::args();
        if args.len() != 3 {
            println!("usage:\n{} /path/to/input /path/to/output",
                     args.next().unwrap());
            std::process::exit(1);
        }
        args.next(); // skip filename
        let f_in = args.next().unwrap();
        let f_out = args.next().unwrap();
        let fd_in = fopen(cstr!(f_in), cstr!("r"));
        let fd_out = fopen(cstr!(f_out), cstr!("w"));

        let buffer: [u8; BUFLEN] = std::mem::uninitialized();
        while !fgets(buffer.as_ptr() as *mut i8, BUFLEN as i32, fd_in).is_null() {
            printf(cstr!("buffer = %s\n"), buffer.as_ptr());
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
