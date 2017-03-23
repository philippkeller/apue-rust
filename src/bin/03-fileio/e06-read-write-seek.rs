/// Exercise 3.6: If you open a file for read–write with the append flag, can you still read
/// from anywhere in the file using lseek? Can you use lseek to replace existing data in the
/// file? Write a program to verify this.
///
/// Answer: yes you can
///
/// $ echo "123456789" > /tmp/e06.txt
/// $ e06-read-write-seek /tmp/e06.txt
/// $ cat /tmp/e06.txt
/// 123hansaplast!

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{SEEK_SET, fopen, fseek, fputs};
use apue::{LibcResult, LibcPtrResult};

fn main() {
    unsafe {
        let file = std::env::args()
            .next_back()
            .expect("specify path to file to be opened for read/write/seek");
        let f = fopen(cstr!(file), cstr!("r+")).check_not_null().expect("fopen failed");
        fseek(f, 3, SEEK_SET).check_not_negative().expect("fseek failed");
        fputs(cstr!("hansaplast!") as _, f);
    }
}
