/// Exercise 10.9: Rewrite the function in Figure 10.14 to handle all the signals from
/// Figure 10.1. The function should consist of a single loop that iterates once for every
/// signal in the current signal mask (not once for every possible signal).
///
/// Takeaway: The assumption is that the position of the bit corresponds
/// to the value of the signal which is true for MacOS. The bit shifting
/// is a bit ugly but I couldn't imagine another way how to execute a code
/// only if a specific signal is set (I found several forum discussions of
/// people who wondered the same thing: on one side you should not mess with
/// the internals of signals, on the other hand you mustn't loop over all
/// possible signals, so then you *need* to somehow access the internals)
///
/// mac only:
/// $ e09-all-signals
/// after setting mask: SIGILL, SIGTRAP, SIGIO,

extern crate apue;
extern crate libc;
extern crate errno;

#[cfg(target_os = "macos")]
mod e09_allsignals {
    use libc::{SIG_SETMASK, sigemptyset, sigaddset};
    use apue::LibcResult;
    use apue::my_libc::sigprocmask;
    use libc::{SIGABRT, SIGALRM, SIGBUS, SIGCHLD, SIGCONT, SIGEMT, SIGFPE, SIGHUP, SIGILL,
               SIGINFO, SIGINT, SIGIO, SIGKILL, SIGPIPE, SIGPROF, SIGQUIT, SIGSEGV, SIGSTOP,
               SIGSYS, SIGTERM, SIGTRAP, SIGTSTP, SIGTTIN, SIGTTOU, SIGURG, SIGUSR1, SIGUSR2,
               SIGVTALRM, SIGWINCH, SIGXCPU, SIGXFSZ};
    use std::collections::HashMap;
    use std::ptr::{null, null_mut};
    use std::mem::uninitialized;

    unsafe fn pr_mask(s: &str, m: HashMap<i32, &str>) {
        let mut sigcur = uninitialized();
        sigprocmask(0, null(), &mut sigcur);
        let mut shift = 1;
        print!("{}: ", s);
        while sigcur != 0 {
            if sigcur & 1 == 1 {
                if let Some(signame) = m.get(&shift) {
                    print!("{}, ", signame);
                }
            }
            sigcur = sigcur >> 1;
            shift += 1;
        }
        println!("");
    }

    macro_rules! insert_map {
        ($m:expr, $s:expr) => {{
            $m.insert($s, stringify!($s))
        }}
    }

    pub fn runit() {
        let mut m = HashMap::new();
        insert_map!(m, SIGHUP);
        insert_map!(m, SIGINT);
        insert_map!(m, SIGQUIT);
        insert_map!(m, SIGILL);
        insert_map!(m, SIGTRAP);
        insert_map!(m, SIGABRT);
        insert_map!(m, SIGEMT);
        insert_map!(m, SIGFPE);
        insert_map!(m, SIGFPE);
        insert_map!(m, SIGKILL);
        insert_map!(m, SIGBUS);
        insert_map!(m, SIGSEGV);
        insert_map!(m, SIGSYS);
        insert_map!(m, SIGPIPE);
        insert_map!(m, SIGALRM);
        insert_map!(m, SIGTERM);
        insert_map!(m, SIGURG);
        insert_map!(m, SIGSTOP);
        insert_map!(m, SIGTSTP);
        insert_map!(m, SIGCONT);
        insert_map!(m, SIGCHLD);
        insert_map!(m, SIGTTIN);
        insert_map!(m, SIGTTOU);
        insert_map!(m, SIGIO);
        insert_map!(m, SIGXCPU);
        insert_map!(m, SIGXFSZ);
        insert_map!(m, SIGVTALRM);
        insert_map!(m, SIGPROF);
        insert_map!(m, SIGWINCH);
        insert_map!(m, SIGINFO);
        insert_map!(m, SIGUSR1);
        insert_map!(m, SIGUSR2);

        unsafe {
            let mut sigs = uninitialized();
            sigemptyset(&mut sigs);
            sigaddset(&mut sigs, SIGIO);
            sigaddset(&mut sigs, SIGILL);
            sigaddset(&mut sigs, SIGTRAP);
            sigprocmask(SIG_SETMASK, &sigs, null_mut())
                .to_option()
                .expect("couldn't set signals");
            pr_mask("after setting mask", m);
        }
    }

}

#[cfg(target_os = "macos")]
fn main() {
    e09_allsignals::runit();
}

#[cfg(not(target_os = "macos"))]
fn main() {
    unimplemented!();
}
