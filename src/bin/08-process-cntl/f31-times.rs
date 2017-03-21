/// Figure 8.31 Time and execute all command-line arguments
///
/// Takeaway: times() returns clock_t which is unsigned for macos.
/// -> cannot check for -1
///
/// $ f31-times "echo hans" 2>&1 | sed 's/0.01/0/g'
/// command: echo hans
/// hans
///   real: 0
///   user: 0
///   sys: 0
///   child user: 0
///   child sys: 0
/// normal termination, exit status = 0

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{setbuf, sysconf, system, clock_t, _SC_CLK_TCK};
use apue::{pr_exit, LibcResult};
use apue::my_libc::{stdout, times, tms};
use std::mem::uninitialized;

unsafe fn do_cmd(cmd: &str) {
    println!("command: {}", cmd);
    let mut tmsstart: tms = uninitialized();
    let mut tmsend: tms = uninitialized();
    // clock_t (return type of times) is unsigned on macos (and probably
    // bsd in general) -> don't check for -1
    let start = times(&mut tmsstart);
    let status = system(cstr!(cmd)).check_not_negative().expect("system() error");
    let end = times(&mut tmsend);
    pr_times(end - start, &tmsstart, &tmsend);
    pr_exit(status);
}

unsafe fn pr_times(real: clock_t, tmsstart: &tms, tmsend: &tms) {
    let clktick: f64 = sysconf(_SC_CLK_TCK).check_not_negative().expect("sysconf error") as f64;
    println!("  real: {}", real as f64 / clktick);
    println!("  user: {}",
             (tmsend.tms_utime - tmsstart.tms_utime) as f64 / clktick);
    println!("  sys: {}",
             (tmsend.tms_stime - tmsstart.tms_stime) as f64 / clktick);
    println!("  child user: {}",
             (tmsend.tms_cutime - tmsstart.tms_cutime) as f64 / clktick);
    println!("  child sys: {}",
             (tmsend.tms_cstime - tmsstart.tms_cstime) as f64 / clktick);
}

fn main() {
    unsafe { setbuf(stdout, std::ptr::null_mut()) };
    let mut args = std::env::args();
    args.next(); // skip exe name
    while let Some(arg) = args.next() {
        unsafe { do_cmd(&arg) };
    }
}
