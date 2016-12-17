/// Figure 8.16 Example of exec functions
///
/// This probably went a bit overboard with cstr! and to_option :) still, worked pretty out
/// of the box (except: first I accidentally called f16-exec and then wondered why fork
/// suddenly dies with "Resource temporarily unavailable"..)
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

use libc::{fork, waitpid, c_char, c_int};
use apue::LibcResult;

extern "C" {
    pub fn execle(__path: *const c_char, __arg0: *const c_char, ...) -> c_int;
    pub fn execlp(__file: *const c_char, __arg0: *const c_char, ...) -> c_int;
}

fn main() {
    unsafe {
        let mut curpath = std::env::current_exe().unwrap();
        curpath.pop();
        curpath.push("f17-echo-all");

        let pid = fork().to_option().expect(&format!("fork error: {}", errno::errno()));
        match pid {
            0 => {
                execle(cstr!(curpath.to_str().unwrap()),
                       cstr!("echoall"),
                       cstr!("myarg1"),
                       cstr!("MY ARG2"),
                       0 as *const c_char,
                       [
                           cstr!("USER=unkown"),
                           cstr!("PATH=/tmp"),
                           0 as *const c_char
                       ].as_ptr())
                    .to_option()
                    .expect(&format!("execle error: {}", errno::errno()));
            }
            _ => {
                waitpid(pid, std::ptr::null_mut(), 0).to_option().expect("wait error");
            }
        }

        let pid = fork().to_option().expect(&format!("fork error: {}", errno::errno()));
        if pid == 0 {
                execlp(cstr!("f17-echo-all"), cstr!("echoall"), cstr!("only 1 arg"),
                0 as *const c_char).to_option().expect(&format!("execlp error: {}", errno::errno()));
        }
    }
}