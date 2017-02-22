#![allow(dead_code)]
/// Figure 11.12 Simplified locking
///
/// Note: 95% is copy-paste from Figure 11.11

extern crate libc;
use libc::{pthread_mutex_t, PTHREAD_MUTEX_INITIALIZER};
use libc::{pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock, pthread_mutex_destroy};
use std::ptr::null;
use std::collections::LinkedList;

const NHASH:i64 = 29;
macro_rules! HASH {
    ($i:expr) => {{
        ($i % NHASH) as usize
    }}
}

struct Foo {
    f_count: i64,
    f_lock: pthread_mutex_t,
    f_id: i64,
}

struct Hashmap {
    fh:Vec<LinkedList<Foo>>,
    hashlock:pthread_mutex_t,
}

impl Hashmap {
    fn new() -> Hashmap {
        let mut fh = Vec::with_capacity(NHASH as usize);
        for i in 0..fh.len() {
            fh[i] = LinkedList::new();
        }
        Hashmap{fh:fh, hashlock: PTHREAD_MUTEX_INITIALIZER}
    }

    fn foo_alloc(&mut self, id: i64) -> Option<&mut Foo> {
        unsafe {
            let mut foo = Foo { f_count: 1, f_lock: std::mem::zeroed(), f_id: id};
            if pthread_mutex_init(&mut foo.f_lock, null()) != 0 {
                // does not need free as foo is dropped upon return
                return None
            }
            pthread_mutex_lock(&mut self.hashlock);
            let mut ll = &mut self.fh[HASH!(id)];
            ll.push_back(foo);
            pthread_mutex_lock(&mut ll.front_mut().unwrap().f_lock);
            pthread_mutex_unlock(&mut self.hashlock);
            // continue initialization
            pthread_mutex_unlock(&mut ll.front_mut().unwrap().f_lock);
            ll.front_mut()
        }
    }

    fn foo_hold(&mut self, foo:&mut Foo) {
        unsafe {
            pthread_mutex_lock(&mut self.hashlock);
            foo.f_count += 1;
            pthread_mutex_unlock(&mut self.hashlock);
        }
    }

    fn foo_find(&mut self, id: i64) -> Option<&mut Foo> {
        unsafe {
            pthread_mutex_lock(&mut self.hashlock);
            for mut foo in &mut self.fh[HASH!(id)] {
                if foo.f_id == id {
                    pthread_mutex_unlock(&mut self.hashlock);
                    return Some(foo);
                }
            }
            pthread_mutex_unlock(&mut self.hashlock);
            None
        }
    }

    fn foo_rele(&mut self, foo:&mut Foo) {
        unsafe {
            pthread_mutex_lock(&mut self.hashlock);
            foo.f_count -= 1;
            if foo.f_count == 0 {
                let ll = self.fh.remove(HASH!(foo.f_id));
                // iter.filter() is probably best way to remove an element from a linked list
                // see https://www.reddit.com/r/rust/comments/33g3ek
                self.fh[HASH!(foo.f_id)] = ll.into_iter().filter(|f| f.f_id != foo.f_id).collect();
                pthread_mutex_unlock(&mut self.hashlock);
                pthread_mutex_destroy(&mut foo.f_lock);
            } else {
                foo.f_count -= 1;
                pthread_mutex_unlock(&mut self.hashlock);
            }
        }
    }
}

fn main() {

}