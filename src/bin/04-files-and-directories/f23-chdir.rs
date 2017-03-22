/// Figure 4.23 Example of chdir function
///
/// $ f23-chdir
/// chdir to /tmp succeeded

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::chdir;
use apue::LibcResult;

fn main() {
    unsafe { chdir(cstr!("/tmp")) }.check_not_negative().expect("chdir failed");
    println!("chdir to /tmp succeeded");
}
