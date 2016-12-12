/// Figure 8.1: Example of fork function
///
/// $ f01-fork | sed 's/pid = [0-9]*, //' # sed is used to hide any pid
/// a write to stdout
/// before fork
/// glob = 6, var = 88
/// before fork
/// glob = 7, var = 89

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{c_int, STDOUT_FILENO, write, fork, usleep, printf, getpid};
use apue::{err_sys, LibcResult};


// though shall not use static mutable..
// used here only to reflect the global var in the C source
// which is an external variable in initialized data
static mut GLOBVAR:i64 = 6;

fn main() {
    unsafe {
        let buf = "a write to stdout\n";
        // automatic variable on the stack
        let mut var: i8 = 88;
        if write(STDOUT_FILENO, buf.as_ptr() as _, buf.len() as _) != buf.len() as _ {
            err_sys("write error");
        }
        // this is line buffered when running in shell (writing to stdout)
        // if stdout is going into a file (e.g. starting from intellij) it is fully buffered
        // and hence output is delayed after fork -> printed twice
        printf(cstr!("before fork\n"));
        if let Some(pid) = fork().to_option() {
            match pid {
                0 => {
                    GLOBVAR += 1;
                    var += 1;
                }
                _ => {
                    usleep(10);
                }
            }
            printf(cstr!("pid = %ld, glob = %d, var = %d\n"), getpid(), GLOBVAR, var as c_int);
        } else {
            err_sys("fork error");
        }
    }
}