/// Figure 3.12: Turn on one or more of the file status flags for a descriptor
/// this example turns on the O_NONBLOCK flag
///
/// Takeaways:
///
/// - Mac lets you set almost every flag (O_READ and O_WRITE), on Linux that's not possible
/// - Mac and Linux have *very* different values for these flags
///
/// List of all flags:
///
/// - Linux: /usr/include/bits/fcntl-linux.h
/// - Mac: /usr/include/sys/fcntl.h
///
/// linux only:
/// $ f12-setfl 5 5<>/tmp/temp.foo
/// current flags: 1000000000000010
/// new flags: 1000100000000010
///
/// mac only:
/// $ f12-setfl 5 5<>/tmp/temp.foo
/// current flags: 10
/// new flags: 110

extern crate libc;
extern crate apue;
#[macro_use(value_t)]
extern crate clap;

use libc::{F_GETFL, F_SETFL, O_NONBLOCK, fcntl};
use clap::App;
use apue::LibcResult;

unsafe fn get_fl(fd: i32) -> std::io::Result<i32> {
    return fcntl(fd, F_GETFL, 0).check_not_negative();
}

unsafe fn set_fl(fd: i32, flags: i32) {
    let val = get_fl(fd).expect("fcntl F_GETFL error");
    println!("current flags: {:b}", val);
    let val = val | flags;
    fcntl(fd, F_SETFL, val).check_not_negative().expect("fcntl F_SETFL error");
}

fn main() {
    let matches = App::new("setfl")
        .args_from_usage("<descr> 'id of the descriptor'")
        .get_matches();
    let fd = value_t!(matches.value_of("descr"), i32).unwrap_or_else(|e| e.exit());
    unsafe {
        set_fl(fd, O_NONBLOCK);
        println!("new flags: {:b}", get_fl(fd).unwrap());
    }
}
