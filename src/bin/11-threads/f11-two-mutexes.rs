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
use libc::{pthread_mutex_init, pthread_mutex_lock};
use std::ptr::null;

const NHASH:usize = 29;
macro_rules! HASH {
    ($i:expr) => {{
        $i % NHASH as i64
    }}
}

#[derive(Clone)]
struct Foo {
    f_count: i64,
    f_lock: pthread_mutex_t,
    f_id: i64,
    f_next: Option<Box<Foo>>,
}

struct Hashmap {
    fh:[Foo;NHASH],
    hashlock:pthread_mutex_t,
}

impl Hashmap {
    fn foo_alloc(&mut self, id: i64) -> Option<Foo> {
        // allocate the object
        unsafe {
            let mut foo = Foo { f_count: 1, f_lock: std::mem::zeroed(), f_id: id, f_next: None };
            if pthread_mutex_init(&mut foo.f_lock, null()) != 0 {
                // does not need free as foo is dropped upon return
                return None
            }
            let idx = HASH!(id);
            pthread_mutex_lock(&mut self.hashlock);
            foo.f_next = Some(Box::new(self.fh[idx as usize]));
        }
        None
    }
}

fn main() {

}

