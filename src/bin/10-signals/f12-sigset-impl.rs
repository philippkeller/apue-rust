/// Figure 10.12 An implementation of sigaddset, sigdelset, and sigismember
///
/// Works only in MacOS, as in other architectures sigset_t is a struct and the bit vector
/// is an internal variable.
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

#[cfg(target_os = "macos")]
mod sigset_impl {
    use libc::sigset_t;
    const NSIG: i8 = 32; // from /usr/include/signal.h

    pub fn sigemptyset() -> sigset_t {
        0 as sigset_t
    }

    pub fn sigbad(signo: i8) -> bool {
        signo <= 0 || signo >= NSIG
    }

    pub fn sigaddset(set: &mut sigset_t, signo: i8) {
        assert!(!sigbad(signo));
        *set |= 1 << (signo - 1); // turn bit on
    }

    pub fn sigdelset(set: &mut sigset_t, signo: i8) {
        assert!(!sigbad(signo));
        *set &= !(1 << signo - 1); // turn bit off
    }

    pub fn sigismember(set: &sigset_t, signo: i8) -> bool {
        assert!(!sigbad(signo));
        set & (1 << (signo - 1)) != 0
    }
}

#[cfg(target_os = "macos")]
use sigset_impl::*;

#[cfg(target_os = "macos")]
fn main() {
    let mut sig = sigemptyset();
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

#[cfg(not(target_os = "macos"))]
fn main() {
    unimplemented!();
}
