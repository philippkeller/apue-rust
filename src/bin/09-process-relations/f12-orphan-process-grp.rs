/// Figure 9.12: Creating an orphaned process group
///
/// This is quite a fun program actually: The child commits suicide,
/// the parent dies as well in a corratoral damage way, but the child,
/// wearing a bullet proof vest survives.
///
/// Takeaway: First I left away fflush (I thought println does that) with
/// the effect that it worked on Linux, but on OSX only the first
/// two lines were printed, the remaining three were only printed when this
/// program was run in parallel or when output is redirected to a file
/// (because it is not buffered).
///
/// The wiring of test.py does not set up a proper tty (yet),
/// therefore only // as commenter of the following example:
///
// $ f12-orphan-process-grp > /tmp/f12-orph.txt
// $ cat /tmp/f12-orph.txt | sed "s/[0-9]//g"
// parent: pid = , ppid = , pgrp = , tpgrp =
// child: pid = , ppid = , pgrp = , tpgrp =
// child: pid = , ppid = , pgrp = , tpgrp =
// read error Input/output error on controlling TTY
// SIGHUP received, pid=

extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;

use libc::{STDIN_FILENO, SIG_ERR, SIGHUP, SIGTSTP, c_int, c_void};
use libc::{printf, getpid, getppid, getpgrp, tcgetpgrp, fflush, fork, sleep, signal, kill, read};
use apue::LibcResult;
use apue::my_libc::stdout;

extern "C" fn sig_hup(_: c_int) {
    unsafe {
        printf(cstr!("SIGHUP received, pid=%ld\n"), getpid());
        fflush(stdout);
    }
}

unsafe fn pr_ids(name: &str) {
    println!("{}: pid = {}, ppid = {}, pgrp = {}, tpgrp = {}",
             name,
             getpid(),
             getppid(),
             getpgrp(),
             tcgetpgrp(STDIN_FILENO));
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
            if signal(SIGHUP, sig_hup as usize) == SIG_ERR {
                panic!("signal error");
            }
            kill(getpid(), SIGTSTP);
            pr_ids("child");
            let s = "0";
            if read(STDIN_FILENO, s.as_ptr() as *mut c_void, 1) != 1 {
                println!("read error {} on controlling TTY", errno::errno());
            }
        }
    }
}