/// Exercise 3.2: Write your own dup2 function that behaves the same way as the dup2 function
/// described in Section 3.12, without calling the fcntl function. Be sure to handle errors
/// correctly.
///
/// $ e02-dup2
/// registered 3
/// registered 4
/// registered 5

extern crate libc;
extern crate apue;

use libc::{dup, close};
use apue::LibcResult;

unsafe fn dup2(fd1: i32, fd2: i32) {
    if fd2 < fd1 {
        panic!("the fd you want is already taken");
    }
    loop {
        let fd = dup(fd1).check_not_negative().expect("error calling dup");
        println!("registered {}", fd);
        if fd >= fd2 {
            break;
        }
    }
}

fn main() {
    unsafe {
        // 3 is stderr, but why are 4 and 5 are already taken..?
        close(3);
        close(4);
        close(5);
        dup2(2, 5);
    }
}
