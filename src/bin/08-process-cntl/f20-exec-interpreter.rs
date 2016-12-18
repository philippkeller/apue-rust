/// Figure 8.20 A program that execs an interpreter file
///
/// doesn't work yet..

extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;

use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;

use libc::{fork, waitpid, c_char, c_int};
use apue::LibcResult;

extern "C" {
    pub fn execl(__path: *const c_char, __arg0: *const c_char, ...) -> c_int;
}


fn main() {
    let curexe = std::env::current_exe().unwrap();
    let testinterp = curexe.with_file_name("testinterp");
    {
        let mut f = OpenOptions::new().create(true).mode(0o777).open(&testinterp)
            .expect("cannot create testinterp");
        f.write_all(b"#!./f17-echo-all\n").unwrap();
    }

    unsafe {
        let pid = fork().to_option().expect("cannot fork");
        match pid {
            0 => {
                execl(cstr!(testinterp.to_str().unwrap()),
                      cstr!("testinterp"), cstr!("myarg1"), cstr!("MY ARG2"),
                      0 as *const c_char)
                    .to_option().expect(&format!("execl error: {}", errno::errno()));
            },
            _ => {
                waitpid(pid, std::ptr::null_mut(), 0).to_option().expect("waitpid error");
            },
        }
    }
}