#![allow(unused_imports)]

/// Figure 11.13 Using pthread_mutex_timedlock
///
/// After finding out that OSX does not support pthread_mutex_timedlock
/// (it would have helped if this info was printed in the book *before* the code example..)
/// the code was quite straightforward, *except* the code for `strerror` which needed to
/// avoid Rusts array bound checking.

extern crate libc;
#[macro_use(as_char, cstr)]
extern crate apue;

#[cfg(target_os = "linux")]
mod timedlock {

    use std::mem::uninitialized;
    use libc::{timespec, c_char, PTHREAD_MUTEX_INITIALIZER, CLOCK_REALTIME};
    use libc::{localtime, pthread_mutex_lock, pthread_mutex_timedlock, clock_gettime};

    use apue::{strerror, array_to_string};
    use apue::my_libc::strftime;

    const BUFLEN: usize = 64;

    unsafe fn print_time(s: &str, tout: &timespec) {
        let tmp = localtime(&tout.tv_sec);
        let buf: [c_char; BUFLEN] = [0; BUFLEN];
        strftime(as_char!(buf), BUFLEN, cstr!("%r"), tmp);
        println!("{} {}", s, array_to_string(&buf));
    }

    pub unsafe fn main() {
        let mut lock = PTHREAD_MUTEX_INITIALIZER;
        pthread_mutex_lock(&mut lock);
        println!("mutex is locked");
        let mut tout: timespec = uninitialized();
        clock_gettime(CLOCK_REALTIME, &mut tout);
        print_time("current time is", &tout);
        tout.tv_sec += 1; // 10 seconds from now on
        // caution: this could lead to deadlock
        let err = pthread_mutex_timedlock(&mut lock, &tout);
        clock_gettime(CLOCK_REALTIME, &mut tout);
        print_time("the time is now", &tout);
        if err == 0 {
            println!("mutex locked again!");
        } else {
            println!("can't lock mutex again: {}", strerror(err));
        }
    }
}

#[cfg(target_os = "linux")]
fn main() {
    unsafe {
        timedlock::main();
    }
}

#[cfg(target_os = "macos")]
fn main() {
    unimplemented!();
}
