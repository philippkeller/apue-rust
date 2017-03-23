/// Figure 8.16 Example of exec functions
///
/// This probably went a bit overboard with cstr! and check_not_negative :) still,
/// worked pretty out of the box (except: first I accidentally called f16-exec and then wondered
/// why fork suddenly dies with "Resource temporarily unavailable"..)
///
/// $ f16-exec | cat | head -5
/// argv[0] = echoall
/// argv[1] = myarg1
/// argv[2] = MY ARG2
/// USER=unkown
/// PATH=/tmp

extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;

use libc::{fork, waitpid, c_char};
use apue::LibcResult;
use apue::my_libc::{execle, execlp};

fn main() {
    unsafe {
        let mut curpath = std::env::current_exe().unwrap();
        curpath.pop();
        curpath.push("f17-echo-all");

        let pid = fork().check_not_negative().expect("fork error");
        match pid {
            0 => {
                execle(cstr!(curpath.to_str().unwrap()),
                       cstr!("echoall"),
                       cstr!("myarg1"),
                       cstr!("MY ARG2"),
                       0 as *const c_char,
                       [cstr!("USER=unkown"), cstr!("PATH=/tmp"), 0 as *const c_char].as_ptr())
                    .check_not_negative()
                    .expect(&format!("execle error: {}", errno::errno()));
            }
            _ => {
                waitpid(pid, std::ptr::null_mut(), 0).check_not_negative().expect("wait error");
            }
        }

        let pid = fork().check_not_negative().expect("fork error");
        if pid == 0 {
            execlp(cstr!("f17-echo-all"),
                   cstr!("echoall"),
                   cstr!("only 1 arg"),
                   0 as *const c_char)
                .check_not_negative()
                .expect(&format!("execlp error: {}", errno::errno()));
        }
    }
}
