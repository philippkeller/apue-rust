#![feature(inclusive_range_syntax)]

/// Exercise 10.6: Write the following program to test the parent–child
/// synchronization functions in Figure 10.24. The process creates a
/// file and writes the integer 0 to the file. The process then calls fork,
/// and the parent and child alternate incrementing the counter in the file.
/// Each time the counter is incremented, print which process (parent or
/// child) is doing the increment.
///
/// Learnings: I had some difficulties to get the while loop with AtomicBool
/// right. I initially tried with fetch_xor until I learned that it *always*
/// returns the previous value.
///
/// $ rm /tmp/e06-sync.txt && e06-sync-parent-child && cat /tmp/e06-sync.txt
/// 200

extern crate apue;
extern crate libc;

use std::io::prelude::*;
use libc::{fork, getppid};
use std::fs::{File, OpenOptions};
use std::io::{Error, SeekFrom};
use apue::LibcResult;
use apue::sync_parent_child::*;

fn increase_file_counter() -> Result<(), Error> {
    let mut f = OpenOptions::new().read(true)
        .write(true)
        .open("/tmp/e06-sync.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    f.seek(SeekFrom::Start(0))?;
    let counter = s.parse::<i32>().unwrap();
    write!(f, "{}", counter + 1)?;
    Ok(())
}

fn main() {
    unsafe {
        {
            let mut f = File::create("/tmp/e06-sync.txt").unwrap();
            f.write_all(b"0").unwrap();
        }
        tell_wait();
        let pid = fork().to_option().expect("fork error");
        if pid == 0 {
            // child
            let ppid = getppid();
            for _ in 1...100 {
                // child goes first
                increase_file_counter().expect("file read/write error");
                tell_parent(ppid);
                wait_parent();
            }
        } else {
            // parent
            for _ in 1...100 {
                wait_child();
                increase_file_counter().expect("file read/write error");
                tell_child(pid);
            }
        }
    }
}
