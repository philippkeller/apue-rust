/// Figure 11.15: using a conditional variable
///
/// Finding: the text for pthread_cond_wait was somehow hard to understand
/// this code made it so much clearer what the function is for

extern crate libc;

use libc::{pthread_cond_t, pthread_mutex_t, PTHREAD_COND_INITIALIZER, PTHREAD_MUTEX_INITIALIZER};
use libc::{pthread_mutex_lock, pthread_cond_wait, pthread_mutex_unlock, pthread_cond_signal};
use std::ptr::null;
use std::collections::VecDeque;

struct Msg {}

struct Messages {
    messages: VecDeque<Msg>,
    qready: pthread_cond_t,
    qlock: pthread_mutex_t,
}

impl Messages {
    fn new() -> Messages {
        Messages {
            messages: VecDeque::new(),
            qready: PTHREAD_COND_INITIALIZER,
            qlock: PTHREAD_MUTEX_INITIALIZER,
        }
    }

    fn process_msg(&mut self) {
        loop {
            unsafe {
                pthread_mutex_lock(&mut self.qlock);
                while self.messages.len() == 0 {
                    pthread_cond_wait(&mut self.qready, &mut self.qlock);
                }
                let mp = self.messages.pop_front();
                pthread_mutex_unlock(&mut self.qlock);
                // now process the message mp
            }
        }
    }

    fn enqueue_msg(&mut self, mp: Msg) {
        unsafe {
            pthread_mutex_lock(&mut self.qlock);
            self.messages.push_front(mp);
            pthread_mutex_unlock(&mut self.qlock);
            pthread_cond_signal(&mut self.qready);
        }

    }
}

fn main() {}
