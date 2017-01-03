/// Figure 9.12: Creating an orphaned process group
///
/// Status: compiles and runs, might be still incorrect though.

extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;

use libc::{STDIN_FILENO, SIGHUP, SIGTSTP, c_int, c_void};
use libc::{printf, getpid, getppid, getpgrp, tcgetpgrp, fflush, fork, sleep, signal, kill, read};
use apue::my_libc::stdout;
use apue::LibcResult;

extern "C" fn sig_hup(_: c_int) {
    unsafe {
        printf(cstr!("SIGHUP received, pid=%ld\n"), getpid());
    }
}

unsafe fn pr_ids(name: &str) {
    println!("{}: pid = {}, ppid = {}, pgrp = {}, tpgrp = {}",
             name,
             getpid(),
             getppid(),
             getpgrp(),
             tcgetpgrp(STDIN_FILENO));
    fflush(stdout);
}

fn main() {
    unsafe {
        pr_ids("parent");
        let pid = fork().to_option().expect("fork error");
        if pid > 0 {
            // parent: sleep to let child stop itself
            sleep(1);
        } else {
            pr_ids("child");
            signal(SIGHUP, sig_hup as usize);
            kill(getpid(), SIGTSTP);
            pr_ids("child");
            let s = "0";
            if read(STDIN_FILENO, s.as_ptr() as *mut c_void, 1) != 1 {
                println!("read error {} on controlling TTY", errno::errno());
            }
        }
    }
}