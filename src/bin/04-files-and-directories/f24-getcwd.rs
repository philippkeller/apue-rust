/// Figure 4.24 Example of getcwd function
///
/// mac only:
/// $ f24-getcwd /var/tmp
/// cwd = "/private/var/tmp"
///
/// linux only:
/// $ f24-getcwd /var/run/
/// cwd = "/run"

extern crate libc;
extern crate apue;
extern crate clap;

use libc::{chdir, getcwd};
use std::ffi::CString;
use apue::{LibcResult, err_sys, path_alloc};
use clap::App;

fn main() {
    unsafe {
        let matches = App::new("fcntl").args_from_usage("<path> path/to/cd/to").get_matches();
        let path = matches.value_of("path").unwrap();
        if let None = chdir(CString::new(path).unwrap().as_ptr()).to_option() {
            err_sys("chdir failed");
        }
        let mut buf = path_alloc();
        if let None = getcwd(buf.as_mut_ptr(), buf.capacity()).to_option() {
            err_sys("getcwd failed");
        }
        println!("cwd = {:?}", CString::from_raw(buf.as_mut_ptr()));
    }
}
