/// Figure 12.8: Using a recursive mutex
///
/// Findings:
/// - Rusts threading library uses pthreads anyway, so no need to use
///   libc's pthread "by hand"
/// - Rusts move semantics come in handy here, so we don't need to malloc
///   at one place and free at another, Rust does that for us
/// - the static mut of course is ugly. I tried:
///   - just handing around the mutex, which causes retry to deadlock
///     (for whatever reason, maybe it is copied at some point or re-initialized
///     so the recursive flag is reset..?)
///   - using Arc and .clone() for the thread, which *would* be the correct
///     way to do this, only Arc does not support mutable values so I would
///     need Mutex in Arc which is a joke, since I would have a
///     Arc<Mutex<pthread_mutex_t>> which would be a mutex within a mutex
///
/// $ f08-recursive-mutex
/// within retry: before lock
/// within retry: after lock
/// within retry: after unlock

extern crate libc;
extern crate apue;

use libc::{PTHREAD_MUTEX_RECURSIVE, PTHREAD_MUTEX_INITIALIZER, pthread_mutex_t};
use libc::{pthread_mutexattr_init, pthread_mutexattr_settype, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock, timespec, usleep};
use apue::LibcResult;
use apue::my_libc::{clock_gettime, CLOCK_REALTIME};
use std::thread;

fn retry(_: i64) {
    unsafe {
        println!("within retry: before lock");
        pthread_mutex_lock(&mut MUTEX);
        println!("within retry: after lock");
        pthread_mutex_unlock(&mut MUTEX);
        println!("within retry: after unlock");
    }
}

struct ToInfo {
    to_fn: fn(i64),
    to_arg: i64,
    to_wait: timespec,
}

fn timeout_helper(tip: ToInfo) {
    unsafe {
        #[cfg(target_os = "macos")]
        libc::nanosleep(&tip.to_wait, std::ptr::null_mut());
        #[cfg(not(target_os = "macos"))]
        libc::clock_nanosleep(CLOCK_REALTIME, 0, tip.to_wait, std::ptr::null());
    }
    (tip.to_fn)(tip.to_arg);
}

unsafe fn timeout(when: &mut timespec, func: fn(i64), arg:i64) {
    let mut now = std::mem::uninitialized();
    clock_gettime(CLOCK_REALTIME, &mut now);
    if (when.tv_sec > now.tv_sec) || (when.tv_sec == now.tv_sec && when.tv_nsec > now.tv_nsec) {
        let mut to_wait = timespec{
            tv_sec: when.tv_sec - now.tv_sec,
            tv_nsec: when.tv_nsec - now.tv_nsec,
        };
        if to_wait.tv_nsec < 0 {
            to_wait.tv_nsec += 1000000000; // 1 second
            to_wait.tv_sec -= 1;
        }
        let tip = ToInfo {
            to_fn: func,
            to_arg: arg,
            to_wait: to_wait,
        };
        thread::spawn(|| {
            timeout_helper(tip);
        });

        return
    }
    // We get here if (a) when <= now, or (b) malloc fails, or
    // (c) we can’t make a thread, so we just call the function now.
    func(arg);
}

static mut MUTEX:pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;

fn main() {
    unsafe {
        let (mut attr, mut when) = std::mem::uninitialized();
        let condition = true;
        pthread_mutexattr_init(&mut attr).check_zero().expect("pthread_mutexattr_init failed");
        pthread_mutexattr_settype(&mut attr, PTHREAD_MUTEX_RECURSIVE).check_zero().expect("can’t set recursive type");
        pthread_mutex_init(&mut MUTEX, &mut attr).check_zero().expect("can’t create recursive mutex");

        // continue processing
        pthread_mutex_lock(&mut MUTEX);
        if condition {
            clock_gettime(CLOCK_REALTIME, &mut when);
            when.tv_nsec += 2000;  /* 1 second from now */
            timeout(&mut when, retry, 0);
        }
        pthread_mutex_unlock(&mut MUTEX);
        usleep(4000);
    }
}