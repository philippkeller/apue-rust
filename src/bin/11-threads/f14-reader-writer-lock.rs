#![allow(dead_code)]

/// Figure 11.14 Using reader–writer locks
///
/// - VecDeque resembles the doubled linked list best which also needs
///   a removal in the middle
/// - interesting that you *need* to consume the output of pthread_rwlock_init
///   otherwise execution blocks
/// - there's also still something wrong because if I remove the println!
///   then the execution hangs
///
/// $ f14-reader-writer-lock
/// kurt
/// kurt
/// kurt

extern crate libc;
extern crate apue;
extern crate errno;

use libc::{pthread_rwlock_t, pthread_t, PTHREAD_RWLOCK_INITIALIZER};
use apue::my_libc::{pthread_rwlock_init};
use libc::{pthread_rwlock_wrlock, pthread_rwlock_unlock, pthread_rwlock_rdlock};

use std::collections::VecDeque;
use std::ptr::null;

struct Queue {
    q: VecDeque<Job>,
    lock: pthread_rwlock_t,
}

struct Job {
    thread_id: pthread_t,
}

// only equals if pointers are equal
impl PartialEq for Job {
    fn eq(&self, other: &Job) -> bool {
        &self == &other
    }
}

impl Queue {
    // queue_init
    fn new() -> Queue {
        let mut q = Queue{q: VecDeque::new(), lock: PTHREAD_RWLOCK_INITIALIZER};
        unsafe{
            if pthread_rwlock_init(&mut q.lock, null()) != 0 {
                println!("fail!");
            }
        };
        q
    }

    fn job_insert(&mut self, job:Job) {
        unsafe{pthread_rwlock_wrlock(&mut self.lock)};
        self.q.push_front(job);
        unsafe{pthread_rwlock_unlock(&mut self.lock)};
    }

    fn job_append(&mut self, job:Job) {
        unsafe{pthread_rwlock_wrlock(&mut self.lock)};
        println!("kurt");
        self.q.push_back(job);
        println!("kurt");
        unsafe{pthread_rwlock_unlock(&mut self.lock)};
        println!("kurt");
    }

    fn job_remove(&mut self, job:&Job) {
        unsafe{pthread_rwlock_wrlock(&mut self.lock)};
        if Some(job) == self.q.front() {
            self.q.pop_front();
        } else if Some(job) == self.q.back() {
            self.q.pop_back();
        } else {
            self.q.retain(|ref x| *x != job);
        }
    }

    fn job_find(&mut self, id:pthread_t) -> Option<&Job> {
        if unsafe{pthread_rwlock_rdlock(&mut self.lock)} != 0 {
            return None;
        }
        let res = if let Some(pos) = self.q.iter().position(|ref x| x.thread_id == id) {
            self.q.get(pos)
        } else {
            None
        };
        unsafe{pthread_rwlock_unlock(&mut self.lock)};
        res
    }
}

fn main() {
    let mut q = Queue::new();
    q.job_append(Job{thread_id:1});
    assert!(q.job_find(1).expect("expected to find 1").thread_id == 1);
}