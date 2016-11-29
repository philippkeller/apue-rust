/// Exercise 3.2: Write your own dup2 function that behaves the same way as the dup2 function
/// described in Section 3.12, without calling the fcntl function. Be sure to handle errors
/// correctly.
///
/// $ e02-dup2
/// registering fd=1
/// registering fd=3
/// registering fd=4

extern crate libc;
extern crate apue;

use libc::dup;
use apue::LibcResult;

unsafe fn dup2(fd1:i32, fd2:i32) {
    if fd2 < fd1 {
        panic!("the fd you want is already taken");
    }
    let mut fd = fd1;
    while fd < fd2 {
        println!("registering fd={}", fd);
        fd = match dup(fd1).to_option() {
            Some(fd) => fd,
            None => panic!("error calling dup")
        }
    }
}

fn main() {
    unsafe {
        dup2(1, 5);
    }
}