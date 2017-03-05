/// Figure 11.16: Using a barrier
///
/// Status: only heapsort part without threads implemented,
/// does not do the right thing as cmp needs to dereference the pointers first

#![feature(rand)]

extern crate libc;
extern crate rand;
extern crate apue;

use apue::my_libc::{heapsort};
use libc::{c_long, c_void, c_int, c_char};
use std::mem;

extern fn cmp(val1: *const c_void, val2: *const c_void) -> c_int {
    let val1 = val1 as c_char;
    let val2 = val2 as c_char;
    if val1 == val2 { 0 }
    else if val1 < val2 {-1}
    else {1}
}

const LEN:usize = 20;

fn main() {
    let mut nums:[c_char;LEN] = unsafe{std::mem::uninitialized()};
    for i in 0..LEN-1 {
        nums[i] = rand::random();
        println!("{}", nums[i])
    }
    let res = unsafe {
        heapsort(nums.as_mut_ptr() as _, LEN, mem::size_of::<c_char>(), cmp)
    };
    println!("result: {}", res);
    for i in 0..LEN-1 {
        println!("{}", nums[i]);
    }

}