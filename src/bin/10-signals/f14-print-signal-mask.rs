/// Figure 10.14 Print the signal mask for the process
///
/// Takeaway: I assumed that SIG_UNBLOCK should set the bit for getting a signal mask bit,
/// but I needed to use SIG_SETMASK to set the SIGINT flag
///
/// $ f14-print-signal-mask
/// after setting mask: SIGINT

extern crate apue;
extern crate libc;
extern crate errno;

use libc::{sigset_t, SIGINT, SIGQUIT, SIGUSR1, SIGALRM, SIG_SETMASK};
use libc::{sigismember, sigaddset};
use apue::LibcResult;
use apue::my_libc::sigprocmask;

macro_rules! print_sig {
    ($set:expr, $s:expr) => {{
        if sigismember($set, $s) == 1 {
            print!(" {}", stringify!($s));
        }
    }}
}

fn pr_mask(s: &str) {
    unsafe {
        let errno_save = errno::errno();
        let mut sigset: sigset_t = std::mem::uninitialized();
        sigprocmask(0, std::ptr::null(), &mut sigset).to_option().expect("sigprocmask error");
        print!("{}", s);
        print_sig!(&sigset, SIGINT);
        print_sig!(&sigset, SIGQUIT);
        print_sig!(&sigset, SIGUSR1);
        print_sig!(&sigset, SIGALRM);
        errno::set_errno(errno_save);
    }
}

fn main() {
    unsafe {
        let mut sigs: sigset_t = std::mem::uninitialized();
        sigaddset(&mut sigs, SIGINT);
        sigprocmask(SIG_SETMASK, &sigs, std::ptr::null_mut())
            .to_option()
            .expect("couldn't set signals");
        pr_mask("after setting mask:");
    }
}
