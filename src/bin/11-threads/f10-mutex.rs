#![allow(dead_code)]
/// Figure 11.10 Using a mutex to protect a data structure
///
/// Of course Rust offers `Arc` and `Mutex` which would offer
/// reference counting and locking out of the box.
/// But the point of this Figure is to figure out how we would
/// do that ourselves.
///
/// On the other hand: instead of `malloc` and `free` Rust
/// does malloc and free for us, so it would be pointless
/// to do that ourselves.
///
/// Let's hope that these functions are gonna be used at some
/// point to check if they really work..

extern crate libc;

use libc::pthread_mutex_t;
use libc::{pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock, pthread_mutex_destroy};
use std::mem::zeroed;
use std::ptr::null;

struct Foo {
    f_count: i64,
    f_lock: pthread_mutex_t,
    f_id: i64,
}

fn foo_alloc(id: i64) -> Option<Foo> {
    unsafe {
        let mut fp = Foo {
            f_count: 1,
            f_lock: zeroed(),
            f_id: id,
        };
        if pthread_mutex_init(&mut fp.f_lock, null()) != 0 {
            return None;
        }
        Some(fp);
    }
    None
}

fn foo_hold(fp: &mut Foo) {
    unsafe {
        pthread_mutex_lock(&mut fp.f_lock);
        fp.f_count += 1;
        pthread_mutex_unlock(&mut fp.f_lock);
    }
}

fn foo_rele(fp: &mut Foo) {
    unsafe {
        pthread_mutex_lock(&mut fp.f_lock);
        fp.f_count -= 1;
        if fp.f_count == 0 {
            // last reference
            pthread_mutex_unlock(&mut fp.f_lock);
            pthread_mutex_destroy(&mut fp.f_lock);
        } else {
            pthread_mutex_unlock(&mut fp.f_lock);
        }
    }
}

fn main() {}
