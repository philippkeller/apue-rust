/// Figure 12.8: Using a recursive mutex

extern crate libc;
extern crate apue;

use libc::{PTHREAD_MUTEX_RECURSIVE};
use libc::{pthread_mutexattr_init, pthread_mutexattr_settype, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock, timespec};
use apue::LibcResult;
use apue::my_libc::{clock_gettime, CLOCK_REALTIME};

fn retry(arg: i64) {
    unimplemented!();
}

fn timeout(when: &mut timespec, func: fn(i64), arg:i64) {
    unimplemented!();
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
            when.tv_sec += 10;  /* 10 seconds from now */
            timeout(&mut when, retry, 0);
        }
        pthread_mutex_unlock(&mut mutex);
    }
}