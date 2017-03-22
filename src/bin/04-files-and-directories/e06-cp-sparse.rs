/// Exercise 4.6: Write a utility like cp(1) that copies a file containing holes, without writing
/// the bytes of 0 to the output file.
///
/// Takeaways:
///
/// - first I mixed up the system calls of fileio and stdio, note to self:
///   + open, read, write, lseek are all fileio (using file descriptors)
///   + fopen, fgetc, fputc, fseek are all stdio (using the *FILE pointer)
/// - lseek to SEEK_HOLE would have been the perfect solution, only this is non-bsd only
/// - a buffer length of 1 is of course silly, best would be a buffer length of the block size:
///   + it would be a lot faster (especially if the data to hole ratio is high)
///   + even if only the first byte of the buffer is non-null it would occupy one block on the disk
///   + the problem would be to check that the whole buffer is consisting of only zeroes
///     http://stackoverflow.com/questions/1493936
///
/// $ echo hans > /tmp/sparse.txt
/// $ truncate -s 1K /tmp/sparse.txt
/// $ echo kurt >> /tmp/sparse.txt
/// $ e06-cp-sparse /tmp/sparse.txt /tmp/sparse2.txt
/// hans
/// seeking to 1025
/// kurt
/// $ rm /tmp/sparse*.txt

extern crate clap;
#[macro_use(cstr)]
extern crate apue;
extern crate libc;
extern crate errno;

use clap::App;
use std::vec::Vec;
use std::io::{self, Write};
use apue::LibcResult;
use libc::{O_RDWR, O_RDONLY, O_CREAT, SEEK_SET, SEEK_CUR, c_void, open, read, write, lseek};

const BUFLEN: usize = 1;

fn main() {
    unsafe {
        let matches = App::new("cp")
            .args_from_usage("<from> source/file\n<to> destination/file")
            .get_matches();
        let from = matches.value_of("from").unwrap();
        let to = matches.value_of("to").unwrap();
        let fd1 = open(cstr!(from), O_RDONLY)
            .check_not_negative()
            .expect(&format!("can't open file {}", from));
        let fd2 = open(cstr!(to), O_RDWR | O_CREAT, 0o600)
            .check_not_negative()
            .expect(&format!("can't open file {}", to));
        let mut buf: Vec<u8> = vec![0; BUFLEN];
        let mut sparse = false;
        while read(fd1, buf.as_mut_ptr() as *mut c_void, BUFLEN) == BUFLEN as _ {
            if buf[0] == 0 {
                // better would be lseek with whence=SEEK_HOLE
                // but this is non-bsd only, so we implement that ourselves
                // of course this is not fast at all
                sparse = true;
                continue;
            }
            // end of a block of zeroes, seek the write file to the right pos
            if sparse {
                let pos = lseek(fd1, 0, SEEK_CUR);
                println!("seeking to {}", pos);
                lseek(fd2, pos, SEEK_SET);
                sparse = false;
            }
            write(fd2, buf.as_ptr() as *mut c_void, BUFLEN);
            io::stdout().write(&buf).unwrap();
        }
    }
}
