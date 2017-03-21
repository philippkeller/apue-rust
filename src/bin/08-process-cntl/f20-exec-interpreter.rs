/// Figure 8.20 A program that execs an interpreter file
///
/// Takeaway: the OpenOption module is quite nice! Took me a while to get the right arguments
/// though
///
/// $ f20-exec-interpreter | head -5 | sed -E 's/[^ ]+(f17|testinterp)//g'
/// argv[0] = -echo-all
/// argv[1] = foo
/// argv[2] =
/// argv[3] = myarg1
/// argv[4] = MY ARG2

extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;

use std::io::prelude::*;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;

use libc::{fork, waitpid, c_char};
use apue::LibcResult;
use apue::my_libc::execl;

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
        let pid = fork().check_not_negative().expect("cannot fork");
        match pid {
            0 => {
                execl(cstr!(testinterp.to_str().unwrap()),
                      cstr!("testinterp"),
                      cstr!("myarg1"),
                      cstr!("MY ARG2"),
                      0 as *const c_char)
                    .check_not_negative()
                    .expect(&format!("execl error: {}", errno::errno()));
            }
            _ => {
                waitpid(pid, std::ptr::null_mut(), 0).check_not_negative().expect("waitpid error");
            }
        }
    }
}
