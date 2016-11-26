/// Figure 3.11: Print file flags for specified descriptor
///
/// $ f11-fcntl 0 < /dev/tty
/// read only
/// $ f11-fcntl 0
/// read write
/// $ f11-fcntl 0 < /dev/tty
/// read only
/// $ f11-fcntl 1 > /tmp/temp.foo
/// $ cat /tmp/temp.foo
/// write only
/// $ f11-fcntl 2 2>>/tmp/temp.foo
/// write only, append
/// $ f11-fcntl 5 5<>/tmp/temp.foo
/// read write

extern crate libc;
extern crate apue;
#[macro_use(value_t)]
extern crate clap;

use libc::{F_GETFL, O_ACCMODE, O_APPEND, O_NONBLOCK, O_SYNC, O_RDONLY, O_WRONLY, O_RDWR, fcntl};
use clap::{App};
use apue::LibcResult;

fn main() {
    unsafe {
        let matches = App::new("fcntl").args_from_usage("<descr> id of the descriptor").get_matches();
        let desc = value_t!(matches.value_of("descr"), u8).unwrap_or_else(|e| e.exit());
        if let Some(val) = fcntl(desc as _, F_GETFL, 0).to_option() {
            let mode = match val & O_ACCMODE {
                O_RDONLY => "read only",
                O_WRONLY => "write only",
                O_RDWR => "read write",
                _ => "unknown access mode"
            };
            print!("{}", mode);
            if val & O_APPEND > 0 {
                print!(", append");
            }
            if val & O_NONBLOCK > 0 {
                print!(", nonblocking");
            }
            if val & O_SYNC > 0 {
                print!(", synchronous writes");
            }
            println!("");
        } else {
            panic!("fcntl error for fd {}", desc);
        }
    }
}