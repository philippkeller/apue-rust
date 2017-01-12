/// Figure 10.10: Calling read with a timeout
///
/// Takeaway: First I tried with the regular `signal` function of libc
/// only to find out that the alarm signal does not interrupt the read
/// call. Digging into the C code it got obvious that the signal function
/// gets overriden by `lib/signal.c` which is a "reliable version of signal(),
/// using POSIX sigaction()". But this function gets only introduced in
/// Figure 10.18. This was quite misleading IMO.
///
/// $ f10-read-timeout 2>&1
/// read error!
/// ERROR: return code 1

extern crate libc;
#[macro_use(as_void)]
extern crate apue;

use libc::{STDOUT_FILENO, STDIN_FILENO, SIGALRM, SIG_ERR, c_int};
use libc::{alarm, write, read, exit};
use apue::signal;

const MAXLINE: usize = 4096;

fn sig_alrm(_: c_int) {
    // nothing to do, just return to interrupt the read
}

fn main() {
    unsafe {
        let line: [u8; MAXLINE] = std::mem::uninitialized();
        if signal(SIGALRM, sig_alrm) == SIG_ERR {
            panic!("signal(SIGALRM) error");
        }
        alarm(1);
        let n = read(STDIN_FILENO, as_void!(line), MAXLINE);
        if n < 0 {
            println!("read error!");
            exit(1);
        }
        alarm(0);
        write(STDOUT_FILENO, as_void!(line), n as _);
    }
}
