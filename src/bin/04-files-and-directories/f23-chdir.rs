/// Figure 4.23 Example of chdir function
///
/// $ f23-chdir
/// chdir to /tmp succeeded

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::chdir;
use apue::{LibcResult, err_sys};

fn main() {
    if let None = unsafe { chdir(cstr!("/tmp")) }.to_option() {
        err_sys("chdir failed");
    }
    println!("chdir to /tmp succeeded");
}
