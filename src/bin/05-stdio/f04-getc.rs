/// Figure 5.4: Copy standard input to standard output using getc and putc
///
/// $ echo hans | f04-getc
/// hans

extern crate libc;
extern crate apue;
use apue::LibcResult;
use apue::my_libc::{stdout, stdin};


fn main() {
    unsafe {
        while let Ok(c) = libc::fgetc(stdin).check_not_negative() {
            if libc::fputc(c, stdout) == libc::EOF {
                panic!("output error");
            }
        }
        if libc::ferror(stdin) != 0 {
            panic!("input error");
        }
    }
}
