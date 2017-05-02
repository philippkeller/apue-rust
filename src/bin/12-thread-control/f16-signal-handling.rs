/// Figure 12.16: Synchronous signal handling
///
/// Findings:
///
/// - instead of pthread_mutex_lock and pthread_cond_signal I found that
///   Rusts Mutex/Condvar serves the same purpose
/// - it took me a while to understand how the mutex works together with
///   pthread_cond_wait (namely, taht pthread_cond_wait unlocks the mutex),
///   although this has been the subject before.. Interesting to see
///   that rusts Mutex/Condvar behave exactly the same:
///   https://doc.rust-lang.org/std/sync/struct.Condvar.html#examples
//
// Does not yet work with test.py, that's why I put it only behind two //
// see http://stackoverflow.com/questions/43730557/

// $ f16-signal-handling & pid=$! && sleep 0.1 && kill -s INT $pid && sleep 0.1 && kill -s QUIT $pid
// interrupt
// quit

extern crate libc;
extern crate apue;

use apue::LibcResult;

use std::sync::{Arc, Mutex, Condvar};
use std::thread;

use libc::{sigset_t, SIGINT, SIGQUIT, SIG_BLOCK, SIG_SETMASK};
use libc::{sigwait, exit, sigemptyset, sigaddset, pthread_sigmask};
use apue::my_libc::sigprocmask;

unsafe fn thr_fn(mask:&sigset_t, pair:Arc<(Mutex<bool>, Condvar)>) {
    let mut signo = std::mem::uninitialized();
    loop {
        sigwait(mask, &mut signo).check_zero().expect("sigwait failed");
        match signo {
            SIGINT => {
                println!("\ninterrupt");
            },
            SIGQUIT => {
                println!("\nquit");
                let &(ref lock, ref cvar) = &*pair;
                let mut quitflag = lock.lock().unwrap();
                *quitflag = true;
                cvar.notify_one();
            },
            _ => {
                println!("unexpectd signal {}", signo);
                exit(1);
            }
        }
    }
}

fn main() {
    unsafe {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();
        let (mut mask, mut oldmask) = std::mem::uninitialized();
        sigemptyset(&mut mask);
        sigaddset(&mut mask, SIGINT);
        sigaddset(&mut mask, SIGQUIT);
        pthread_sigmask(SIG_BLOCK, &mask, &mut oldmask).check_zero().expect("SIG_BLOCK error");
        thread::spawn(move || {
            thr_fn(&mask, pair2);
        });
        let &(ref lock, ref cvar) = &*pair;

        // wait for the quitflag
        // trick is that the lock is unlocked on cvar.wait(quitflag)
        {
            let mut quitflag = lock.lock().unwrap();
            while !*quitflag {
                quitflag = cvar.wait(quitflag).unwrap();
            }
            *quitflag = false;
        }

        // SIGQUIT has been caught. Lock is unlocked upon dropping of the quitflag var
        // reset signal mask which unblocks SIGQUIT
        sigprocmask(SIG_SETMASK, &oldmask, std::ptr::null_mut()).check_not_negative().expect("SIG_SETMASK error");
    }
}