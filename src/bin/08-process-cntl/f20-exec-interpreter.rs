/// Figure 8.20 A program that execs an interpreter file
///
/// Takeaway: the OpenOption module is quite nice! Took me a while to get the right arguments
/// though
///
/// $ f20-exec-interpreter | head -5
/// argv[0] = /Users/philipp/oss/apue/target/debug/f17-echo-all
/// argv[1] = foo
/// argv[2] = /Users/philipp/oss/apue/target/debug/testinterp
/// argv[3] = myarg1
/// argv[4] = MY ARG2


extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;

use std::io::prelude::*;
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
        let mut f = OpenOptions::new()
            .mode(0o777)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&testinterp)
            .expect("cannot create testinterp");
        f.write_all(format!("#!{} foo\n",
                               curexe.with_file_name("f17-echo-all").to_str().unwrap())
                .as_bytes())
            .unwrap();
    }

    unsafe {
        let pid = fork().to_option().expect("cannot fork");
        match pid {
            0 => {
                execl(cstr!(testinterp.to_str().unwrap()),
                      cstr!("testinterp"),
                      cstr!("myarg1"),
                      cstr!("MY ARG2"),
                      0 as *const c_char)
                    .to_option()
                    .expect(&format!("execl error: {}", errno::errno()));
            }
            _ => {
                waitpid(pid, std::ptr::null_mut(), 0).to_option().expect("waitpid error");
            }
        }
    }
}
