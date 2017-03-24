/// Figure 8.3: Example of vfork function
///
/// Findings:
/// - when calling exit() this error is triggered:
///   f03-vfork(17135,0x7fffabbdc3c0) malloc:
///   *** error for object 0x7fc0b4c02830: pointer being freed was not allocated
///   I *think* it's because the rust cleanup tries to free a memory twice, once for every
///   process
/// - when using a normal stack var (no Box), and compiling this in release
///   mode, the parent does not see the var += 1 (but it DOES see the GLOBVAR +=1).
///   The child *does* do the += 1 (printf shows it), so it is not compiled away,
///   there's some other strange effect at hand which I don't understand.
///   Anyway, changing into a Box solves the issue
///
/// $ f03-vfork | sed 's/pid = [0-9]*, //'
/// before vfork
/// glob = 7, var = 89

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{printf, getpid, c_int, _exit};
use apue::{LibcResult};
use apue::my_libc::vfork;

static mut GLOBVAR: i64 = 6;

#[allow(unused_assignments)]
fn main() {
    unsafe {
        let mut var = Box::new(88);
        printf(cstr!("before vfork\n"));
        match vfork().check_not_negative().expect("vfork error") {
            0 => {
                // child
                GLOBVAR += 1;
                *var += 1;
                _exit(0);
            }
            _ => {
                printf(cstr!("pid = %ld, glob = %d, var = %d\n"),
                       getpid(),
                       GLOBVAR,
                       *var as c_int);
            }
        }
    }
}
