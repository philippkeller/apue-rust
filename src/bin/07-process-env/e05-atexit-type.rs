/// Exercise 7.5: Use the typedef facility of C to define a new data type Exitfunc for an exit
/// handler. Redo the prototype for atexit using this data type.
///
/// $ e05-atexit-type
/// main is done
/// first exit handler
/// first exit handler
/// second exit handler

extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;

use apue::LibcResult;
use libc::{atexit, printf};

type Exitfunc = extern "C" fn();

fn my_atexit(f: Exitfunc) -> std::io::Result<i32> {
    unsafe { atexit(f).check_not_negative() }
}

extern "C" fn my_exit1() {
    unsafe { printf(cstr!("first exit handler\n")) };
}

extern "C" fn my_exit2() {
    unsafe { printf(cstr!("second exit handler\n")) };
}

fn main() {
    my_atexit(my_exit2).expect(&format!("can't register my_exit2: {}", errno::errno()));
    my_atexit(my_exit1).expect(&format!("can't register my_exit1: {}", errno::errno()));
    my_atexit(my_exit1).expect(&format!("can't register my_exit1: {}", errno::errno()));
    println!("main is done");
}
