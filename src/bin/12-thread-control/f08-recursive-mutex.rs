/// Figure 12.8: Using a recursive mutex
///
/// Findings:
/// - Rusts threading library uses pthreads anyway, so no need to use
///   libc's pthread "by hand"
/// - Rusts move semantics come in handy here, so we don't need to malloc
///   at one place and free at another, Rust does that for us
/// - Currently doesn't work on OSX: it locks at pthread_mutex_lock
///   within the retry function

extern crate libc;
extern crate apue;

use libc::{PTHREAD_MUTEX_RECURSIVE, pthread_mutex_t};
use libc::{pthread_mutexattr_init, pthread_mutexattr_settype, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock, timespec, usleep};
use apue::LibcResult;
use apue::my_libc::{clock_gettime, CLOCK_REALTIME};
use std::sync::Arc;
use std::thread;

fn retry(arg: i64, mut mutex: pthread_mutex_t) {
    unsafe {
        println!("within retry: before lock");
        pthread_mutex_lock(&mut mutex);
        println!("within retry: after lock");
        pthread_mutex_unlock(&mut mutex);
        println!("within retry: after unlock");
    }
}

struct ToInfo {
    to_fn: fn(i64, pthread_mutex_t),
    to_arg: i64,
    to_wait: timespec,
    to_mutex: pthread_mutex_t,
}

fn timeout_helper(tip: ToInfo) {
    unsafe {
        #[cfg(target_os = "macos")]
        libc::nanosleep(&tip.to_wait, std::ptr::null_mut());
        #[cfg(not(target_os = "macos"))]
        libc::clock_nanosleep(CLOCK_REALTIME, 0, tip.to_wait, std::ptr::null());
    }
    (tip.to_fn)(tip.to_arg, tip.to_mutex);
}

unsafe fn timeout(when: &mut timespec, func: fn(i64, pthread_mutex_t), arg:i64, mutex: pthread_mutex_t) {
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
            to_mutex: mutex,
        };
        thread::spawn(|| {
            timeout_helper(tip);
        });

        return
    }
    // We get here if (a) when <= now, or (b) malloc fails, or
    // (c) we can’t make a thread, so we just call the function now.
    func(arg, mutex);
}

fn main() {
    unsafe {
        let (mut attr, mut mutex, mut when) = std::mem::uninitialized();
        let condition = true;
        pthread_mutexattr_init(&mut attr).check_zero().expect("pthread_mutexattr_init failed");
        pthread_mutexattr_settype(&mut attr, PTHREAD_MUTEX_RECURSIVE).check_zero().expect("can’t set recursive type");
        pthread_mutex_init(&mut mutex, &mut attr).check_zero().expect("can’t create recursive mutex");

        // continue processing
        pthread_mutex_lock(&mut mutex);
        if condition {
            clock_gettime(CLOCK_REALTIME, &mut when);
            when.tv_sec += 1;  /* 1 second from now */
            timeout(&mut when, retry, 0, mutex);
        }
        pthread_mutex_unlock(&mut mutex);
        usleep(2000000);
    }
}