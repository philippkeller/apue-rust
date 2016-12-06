/// Figure 7.3 Example of exit handlers
///
/// Takeaway: it seems that at the point of running the exit handlers, rusts destructors
/// have already run and things like println!() are no longer available (calling them results in
/// `pointer being freed was not allocated`), see http://stackoverflow.com/questions/35980148
///
/// $ f03-atexit
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

extern "C" fn my_exit1() {
    unsafe { printf(cstr!("first exit handler\n")) };
}

extern "C" fn my_exit2() {
    unsafe { printf(cstr!("second exit handler\n")) };
}

fn main() {
    unsafe {
        atexit(my_exit2)
            .to_option()
            .expect(&format!("can't register my_exit2: {}", errno::errno()));
        atexit(my_exit1)
            .to_option()
            .expect(&format!("can't register my_exit1: {}", errno::errno()));
        atexit(my_exit1)
            .to_option()
            .expect(&format!("can't register my_exit1: {}", errno::errno()));
        println!("main is done");
    }
}
