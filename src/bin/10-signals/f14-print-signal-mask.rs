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

use libc::{sigset_t, SIGINT, SIG_SETMASK, sigaddset};
use apue::{LibcResult, pr_mask};
use apue::my_libc::sigprocmask;

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
