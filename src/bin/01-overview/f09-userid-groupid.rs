/// Figure 1.9: Print user ID and group ID
///
/// $ f09-userid-groupid > /dev/null

extern crate libc;

use libc::{getuid, getgid};

fn main() {
    unsafe {
        println!("uid={:?}, gid={:?}", getuid(), getgid());
    }
}