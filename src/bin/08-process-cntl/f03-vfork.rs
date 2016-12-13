/// Figure 8.3: Example of vfork function
///
/// $ f03-vfork | sed 's/pid = [0-9]*, //'
/// before vfork
/// glob = 7, var = 89

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{printf, _exit, getpid, c_int};
use apue::{LibcResult, err_sys};


// vfork is not implemented in libc, and that's probably good so
// as vfork is somehow deprecated
extern "C" {
    pub fn vfork() -> libc::pid_t;
}

static mut GLOBVAR: i64 = 6;

fn main() {
    unsafe {
        let mut var: i8 = 88;
        printf(cstr!("before vfork\n"));
        if let Some(pid) = vfork().to_option() {
            match pid {
                0 => {
                    // child
                    GLOBVAR += 1;
                    var += 1;
                    _exit(0);
                }
                _ => {
                    printf(cstr!("pid = %ld, glob = %d, var = %d\n"),
                           getpid(),
                           GLOBVAR,
                           var as c_int);
                }
            }
        } else {
            err_sys("vfork error");
        }
    }
}
