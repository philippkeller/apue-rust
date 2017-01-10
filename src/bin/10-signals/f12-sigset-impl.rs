/// Figure 10.12 An implementation of sigaddset, sigdelset, and sigismember
///
/// This was a bit pointless really (why is a code fragment about bit
/// masking in a book which teaches Unix?), only converted for completeness.
///
/// $ f12-sigset-impl
/// 0b10000
/// 0b10000
/// bit 5 is set
/// 0b0

extern crate apue;
extern crate libc;
extern crate errno;

use libc::{sigset_t};

#[cfg(target_os = "macos")]
const NSIG:i8 = 32; // from /usr/include/signal.h
#[cfg(target_os = "linux")]
const NSIG:i8 = 64; // from /usr/include/x86_64-linux-gnu/bits/signum.h

fn sigbad(signo:i8) -> bool {
    signo <= 0 || signo >= NSIG
}

fn sigaddset(set:&mut sigset_t, signo:i8) {
    assert!(!sigbad(signo));
    *set |= 1 << (signo - 1); // turn bit on
}

fn sigdelset(set:&mut sigset_t, signo:i8) {
    assert!(!sigbad(signo));
    *set &= !(1 << signo - 1); // turn bit off
}

fn sigismember(set:&sigset_t, signo:i8) -> bool {
    assert!(!sigbad(signo));
    set & (1 << (signo - 1)) != 0
}

fn main() {
    let mut sig = 0;
    sigaddset(&mut sig, 5);
    println!("{:#b}", sig);
    sigdelset(&mut sig, 4);
    println!("{:#b}", sig);
    if sigismember(&sig, 5) {
        println!("bit 5 is set");
    }
    sigdelset(&mut sig, 5);
    println!("{:#b}", sig);
}