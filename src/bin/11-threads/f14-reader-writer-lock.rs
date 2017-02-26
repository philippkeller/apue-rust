/// Figure 11.14 Using reader–writer locks
///
/// - VecDeque resembles the doubled linked list best which also needs
///   a removal in the middle
///
/// Status: only stub

extern crate libc;

use libc::{pthread_rwlock_t, pthread_t};

use std::collections::VecDeque;

struct Queue {
    q: VecDeque<pthread_t>,
    lock: pthread_rwlock_t,
}

fn main() {

}