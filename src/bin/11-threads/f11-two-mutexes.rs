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

struct Foo<'a> {
    f_count: i64,
    f_lock: pthread_mutex_t,
    f_id: i64,
    f_next: Option<Box<&'a Foo<'a>>>,
}

struct Hashmap<'a> {
    fh:Vec<Foo<'a>>,
    hashlock:pthread_mutex_t,
}

impl <'a>Hashmap<'a> {
    fn new() -> Hashmap<'a> {
        let mut fh = Vec::with_capacity(NHASH);
        for i in 0..NHASH {
            fh[i] = Foo {f_count: 1, f_lock: PTHREAD_MUTEX_INITIALIZER, f_id: -1, f_next: None};
        }
        Hashmap{fh:fh, hashlock: PTHREAD_MUTEX_INITIALIZER}
    }

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
            foo.f_next = Some(Box::new(&self.fh[idx as usize]));
        }
        None
    }
}

fn main() {

}

