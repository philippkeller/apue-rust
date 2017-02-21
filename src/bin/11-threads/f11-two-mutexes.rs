/// Figure 11.11 Using two mutexes
///
/// Findings:
/// - After wrestling with static mut again realized
///   that fh and hashlock are much better off stuffed into
///   a struct
///
/// Status: only partially implemented

extern crate libc;
use libc::{pthread_mutex_t, PTHREAD_MUTEX_INITIALIZER};
use libc::{pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock};
use std::ptr::null;
use std::collections::LinkedList;

const NHASH:usize = 29;
macro_rules! HASH {
    ($i:expr) => {{
        $i % NHASH as i64
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
        let mut fh = Vec::with_capacity(NHASH);
        for i in 0..NHASH {
            fh[i] = LinkedList::new();
        }
        Hashmap{fh:fh, hashlock: PTHREAD_MUTEX_INITIALIZER}
    }

    fn foo_alloc(&mut self, id: i64) -> Option<&Foo> {
        // allocate the object
        unsafe {
            let mut foo = Foo { f_count: 1, f_lock: std::mem::zeroed(), f_id: id};
            if pthread_mutex_init(&mut foo.f_lock, null()) != 0 {
                // does not need free as foo is dropped upon return
                return None
            }
            let idx = HASH!(id);
            pthread_mutex_lock(&mut self.hashlock);
            let mut ll = &mut self.fh[idx as usize];
            ll.push_back(foo);
            pthread_mutex_lock(&mut ll.front_mut().unwrap().f_lock);
            pthread_mutex_unlock(&mut self.hashlock);
            // continue initialization
            pthread_mutex_unlock(&mut ll.front_mut().unwrap().f_lock);
            Some(&foo)
        }
    }
}

fn main() {

}

