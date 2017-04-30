/// Figure 12.12: A reentrant (thread-safe) version of getenv
///
/// Findings:
/// - why would the caller need to do his own buffer? Even in C you could
///   just malloc a buffer for the caller, no? Of course the caller would
///   need to free it up, but isn't that normal in C that you need take
///   care of buffers that "others" did malloc for you?

extern crate libc;
extern crate apue;

use std::ffi::CStr;

use libc::{pthread_mutex_t, PTHREAD_MUTEX_INITIALIZER, PTHREAD_MUTEX_RECURSIVE, c_char};
use libc::{pthread_mutexattr_init, pthread_mutexattr_settype, pthread_mutex_init, pthread_mutexattr_destroy, pthread_mutex_lock, pthread_mutex_unlock};

use apue::my_libc::{pthread_once, pthread_once_t, PTHREAD_ONCE_INIT};

static mut ENV_MUTEX:pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
static mut INIT_DONE:pthread_once_t = PTHREAD_ONCE_INIT;

extern "C" {
    pub static environ: *const *const c_char;
}

extern "C" fn thread_init() {
    unsafe {
        let mut attr = std::mem::uninitialized();
        pthread_mutexattr_init(&mut attr);
        pthread_mutexattr_settype(&mut attr, PTHREAD_MUTEX_RECURSIVE);
        pthread_mutex_init(&mut ENV_MUTEX, &attr);
        pthread_mutexattr_destroy(&mut attr);
    }
}

fn getenv_r(name:&str) -> Option<&str> {
    unsafe {
        pthread_once(&mut INIT_DONE, thread_init);
        pthread_mutex_lock(&mut ENV_MUTEX);
        let mut cmp = name.to_owned();
        cmp.push_str("=");
        let mut i = 0isize;
        loop {
            if *environ.offset(i) == std::ptr::null() {
                break
            }
            let s = CStr::from_ptr(*(environ.offset(i as _))).to_str().expect("no valid string");
            if s.starts_with(&cmp) {
                pthread_mutex_unlock(&mut ENV_MUTEX);
                return Some(s);
            }
            i += 1;
        }
        pthread_mutex_unlock(&mut ENV_MUTEX);
        None
    }
}

fn main() {
    println!("{}", getenv_r("PATH").unwrap());
}