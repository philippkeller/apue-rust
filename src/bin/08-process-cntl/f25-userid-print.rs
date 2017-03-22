/// Figure 8.25 Print real and effective user IDs
///
/// $ f25-userid-print | sed 's/[0-9]//g' # remove ids as they're different on every setup
/// real uid = , effective uid =

extern crate libc;

use libc::{getuid, geteuid};

fn main() {
    unsafe {
        println!("real uid = {}, effective uid = {}", getuid(), geteuid());
    }
}
