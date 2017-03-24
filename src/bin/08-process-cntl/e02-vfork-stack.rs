/// Exercise 8.2: Recall the typical arrangement of memory in Figure 7.6.
/// Because the stack frames corresponding to each function call are usually
/// stored in the stack, and because after a vfork the child runs in the
/// address space of the parent, what happens if the call to vfork is from a
/// function other than main and the child does a return from this function
/// after the vfork? Write a test program to verify this, and draw a picture
/// of what’s happening.
///
/// Takeaway: this is a good showcase why vfork is very very VERY
/// dangerous. What happens is that the process which exits the function
/// tears down the stack frame, pulling the rog out under the other processes
/// feet. If he then tries to access a stack var, this is bye bye birdie.

extern crate libc;
extern crate apue;

use apue::my_libc::vfork;
use apue::LibcResult;
use libc::sleep;

fn callme() {
    let pid = unsafe { vfork() }.check_not_negative().expect("vfork failed");
    let b = 2;
    match pid {
        0 => {
            // child immediately returns
        }
        _ => {
            // parent waits a second and tries to access stack var b
            unsafe { sleep(1) };
            println!("b={}", b);
        }
    }
}

fn main() {
    callme();
}
