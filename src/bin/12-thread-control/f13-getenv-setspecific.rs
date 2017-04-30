/// Figure 12.13: A thread-safe, compatible version of getenv
///
/// Findings:
/// - I was surprised that the copying around with *(envbuf.offset())
///   worked at first try and that calling `free` via destructor
///   does not interfer with Rusts thread logic
/// - I first thought PTHREAD_ONCE_INIT is same in Linux as it is on OSX
///   but I was wrong, in Linux it's 0, whereas on OSX it's 0x30B1BCBA
///   Cost me about 1.5h to find out why on Linux this program always threw
///   segfaults because pthread_getspecific was returning a pointer which
///   sometimes, when writing to pointer + offset just crashed in a segfault..
///
/// $ f13-getenv-setspecific | sed 's/=.*//g'
/// PATH
/// PATH
/// PATH
/// PATH
/// PATH

extern crate libc;
extern crate apue;

use std::ffi::CStr;
use std::ptr::null_mut;
use std::thread;

use libc::{pthread_mutex_t, PTHREAD_MUTEX_INITIALIZER, c_char, pthread_key_t};
use libc::{pthread_mutex_lock, pthread_mutex_unlock, pthread_key_create, malloc, free, pthread_getspecific, pthread_setspecific};

use apue::my_libc::{pthread_once, pthread_once_t, PTHREAD_ONCE_INIT};

static mut ENV_MUTEX:pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
static mut INIT_DONE:pthread_once_t = PTHREAD_ONCE_INIT;
static mut KEY:pthread_key_t = 0;
const MAXSTRINGSZ:usize = 4096;

extern "C" {
    pub static environ: *const *const c_char;
}

extern "C" fn thread_init() {
    unsafe {
        pthread_key_create(&mut KEY, Some(free)); // no need to call free, tidying up is done by Rust
    }
}

fn getenv(name:&str) -> Option<*mut i8> {
    unsafe {
        pthread_once(&mut INIT_DONE, Some(thread_init));
        pthread_mutex_lock(&mut ENV_MUTEX);
        let mut envbuf = pthread_getspecific(KEY) as *mut c_char;
        if envbuf == null_mut() {
            envbuf = malloc(MAXSTRINGSZ) as *mut c_char;
            if envbuf == null_mut() {
                pthread_mutex_unlock(&mut ENV_MUTEX);
                return None;
            }
            pthread_setspecific(KEY, envbuf as _);
        }
        let mut cmp = name.to_owned();
        cmp.push_str("=");
        let mut i = 0isize;
        loop {
            if *environ.offset(i) == std::ptr::null() {
                break
            }
            let s = CStr::from_ptr(*(environ.offset(i as _))).to_str().expect("no valid string");
            if s.starts_with(&cmp) {
                for (j, c) in s.chars().enumerate() {
                    *(envbuf.offset(j as _)) = c as _;
                }
                *(envbuf.offset(s.len() as _)) = 0 as _;
                pthread_mutex_unlock(&mut ENV_MUTEX);
                return Some(envbuf);
            }
            i += 1;
        }
        pthread_mutex_unlock(&mut ENV_MUTEX);
        None
    }
}

fn main() {
    let mut threads = vec![];
    for _ in 0..5 {
        threads.push(thread::spawn(|| {
            let s = unsafe{CStr::from_ptr(getenv("PATH").unwrap())};
            println!("{}", s.to_str().unwrap());
        }));
    }
    for thread in threads {
        let _ = thread.join();
    }
}