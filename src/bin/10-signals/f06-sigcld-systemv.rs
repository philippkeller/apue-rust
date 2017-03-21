/// Figure 10.6 System V SIGCLD handler that doesn’t work
///
/// linux only:
/// $ f06-sigcld-systemv | sed 's/[0-9]*//g'
/// SIGCLD received
/// pid =

extern crate libc;
extern crate apue;

// only Linux defines SIGCLD, it is equal to SIGCHLD
#[cfg(target_os = "linux")]
mod sigcld {
    use libc::{c_int, SIGCHLD, SIG_ERR};
    use libc::{signal, wait, fork, usleep, _exit, pause};
    use apue::LibcResult;

    const SIGCLD: c_int = SIGCHLD;

    unsafe fn sig_cld(_: c_int) {
        println!("SIGCLD received");
        if signal(SIGCLD, sig_cld as usize) == SIG_ERR {
            panic!("signal error");
        }
        let mut status: c_int = 0;
        let pid = wait(&mut status).check_not_negative().expect("wait error");
        println!("pid = {}", pid);
    }

    pub fn doit() {
        unsafe {
            if signal(SIGCLD, sig_cld as usize) == SIG_ERR {
                panic!("signal error");
            }
            let pid = fork().check_not_negative().expect("fork error");
            if pid == 0 {
                // child
                usleep(100);
                _exit(0);
            } else {
                // parent
                pause();
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn main() {
    sigcld::doit();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    unimplemented!();
}
