#![allow(dead_code)]
/// Figure 11.11 Using two mutexes
///
/// Of course, you would not reimplement this as Rust has Arc and Mutex
/// which would make the whole excercise boil down to <10 lines
///
/// Findings:
/// - After wrestling with static mut again realized
///   that fh and hashlock are much better off stuffed into
///   a struct
/// - After wrestling with lifetimes (especially in `foo_alloc`)
///   and opening http://stackoverflow.com/questions/42392250
///   it was quite easy and uses no explicit lifetimes (all lifetimes
///   are alluded)
/// - removing elements from linked lists is not supported by the
///   std lib even though I can't see why it wouldn't (as it's basically
///   the same as `remove` in the Vec)
///
/// Status: compiles, not tested. I'd especially like to see if/how
/// the freeing of foo in `rele` works

extern crate libc;
use libc::{pthread_mutex_t, PTHREAD_MUTEX_INITIALIZER};
use libc::{pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock, pthread_mutex_destroy};
use std::ptr::null;
use std::collections::LinkedList;

const NHASH: i64 = 29;
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
    fh: Vec<LinkedList<Foo>>,
    hashlock: pthread_mutex_t,
}

impl Hashmap {
    fn new() -> Hashmap {
        let mut fh = Vec::with_capacity(NHASH as usize);
        for i in 0..fh.len() {
            fh[i] = LinkedList::new();
        }
        Hashmap {
            fh: fh,
            hashlock: PTHREAD_MUTEX_INITIALIZER,
        }
    }

    fn foo_alloc(&mut self, id: i64) -> Option<&mut Foo> {
        unsafe {
            let mut foo = Foo {
                f_count: 1,
                f_lock: std::mem::zeroed(),
                f_id: id,
            };
            if pthread_mutex_init(&mut foo.f_lock, null()) != 0 {
                // does not need free as foo is dropped upon return
                return None;
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

    fn foo_hold(foo: &mut Foo) {
        unsafe {
            pthread_mutex_lock(&mut foo.f_lock);
            foo.f_count += 1;
            pthread_mutex_unlock(&mut foo.f_lock);
        }
    }

    fn foo_find(&mut self, id: i64) -> Option<&mut Foo> {
        unsafe {
            pthread_mutex_lock(&mut self.hashlock);
            for mut foo in &mut self.fh[HASH!(id)] {
                if foo.f_id == id {
                    Hashmap::foo_hold(&mut foo);
                    pthread_mutex_unlock(&mut self.hashlock);
                    return Some(foo);
                }
            }
            pthread_mutex_unlock(&mut self.hashlock);
            None
        }
    }

    fn foo_rele(&mut self, foo: &mut Foo) {
        unsafe {
            pthread_mutex_lock(&mut foo.f_lock);
            if foo.f_count == 1 {
                pthread_mutex_unlock(&mut foo.f_lock);
                pthread_mutex_lock(&mut self.hashlock);
                pthread_mutex_lock(&mut foo.f_lock);
                // need to recheck the condition
                if foo.f_count != 1 {
                    foo.f_count -= 1;
                    pthread_mutex_unlock(&mut foo.f_lock);
                    pthread_mutex_unlock(&mut self.hashlock);
                    return;
                }
                let ll = self.fh.remove(HASH!(foo.f_id));
                // iter.filter() is probably best way to remove an element from a linked list
                // see https://www.reddit.com/r/rust/comments/33g3ek
                self.fh[HASH!(foo.f_id)] = ll.into_iter().filter(|f| f.f_id != foo.f_id).collect();
                pthread_mutex_unlock(&mut self.hashlock);
                pthread_mutex_unlock(&mut foo.f_lock);
                pthread_mutex_destroy(&mut foo.f_lock);
            } else {
                foo.f_count -= 1;
                pthread_mutex_unlock(&mut foo.f_lock);
            }
        }
    }
}

fn main() {}
