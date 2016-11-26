/// Figure 3.5: Copy standard input to standard output
///
/// Takeaways:
///
/// - there's a difference when doing `cat file.dat | myprog` or `myprog < file.dat`:
///   with the pipe the data goes through the pipe buffer and this might cap the max
///   read buffer size, see http://unix.stackexchange.com/a/11954/168663
/// - writing to /tmp/discard.txt is a lot slower than writing to /dev/null. This of
///   course is obvious. But the difference is big: with a buffer size of 1 writing
///   into /tmp/discard.txt took 23.5 minutes. Writing to /dev/null only 8.5 minutes.
///
/// ## Timing
///
/// > dd if=/dev/zero of=bigfile.dat bs=516581760 count=1
///
/// Test script:
///
/// ```
/// i=1
/// while [[ $i -le 1000000 ]]; do
///     echo "$i"
///     time `./f05-copy-stdin-stdout $i < bigfile.dat > /dev/null`
///     ((i = i * 2))
/// done
/// ```
///
/// ### Mac OSX
///
/// > stat -f '%k'  bigfile.dat
/// 4096
/// > uname -prsv
/// Darwin 16.1.0 Darwin Kernel Version 16.1.0: Thu Oct 13 21:26:57 PDT 2016;
/// root:xnu-3789.21.3~60/RELEASE_X86_64 i386
///
/// | buffsize | num loops | real time | user time | sys time |
/// |----------|-----------|-----------|-----------|----------|
/// |        1 | 516581760 |    504.16 |    134.99 |   363.42 |
/// |        2 | 258290880 |    255.55 |     70.33 |   183.28 |
/// |        4 | 129145440 |    125.47 |     33.79 |    90.71 |
/// |        8 |  64572720 |     63.15 |     16.84 |    45.81 |
/// |       16 |  32286360 |     31.55 |      8.40 |    22.91 |
/// |       32 |  16143180 |     15.80 |      4.22 |    11.46 |
/// |       64 |   8071590 |      7.89 |      2.10 |     5.73 |
/// |      128 |   4035795 |      3.99 |      1.08 |     2.88 |
/// |      256 |   2017898 |      2.02 |      0.53 |     1.48 |
/// |      512 |   1008949 |      1.06 |      0.26 |     0.78 |
/// |     1024 |    504475 |      0.58 |      0.13 |     0.43 |
/// |     2048 |    252238 |      0.33 |      0.06 |     0.26 |
/// |     4096 |    126119 |      0.21 |      0.03 |     0.16 |
/// |     8192 |     63060 |      0.14 |      0.01 |     0.12 |
/// |    16384 |     31530 |      0.12 |      0.01 |     0.10 |
/// |    32768 |     15765 |      0.11 |      0.00 |     0.10 |
/// |    65536 |      7883 |      0.11 |      0.00 |     0.10 |
/// |   131072 |      3942 |      0.11 |      0.00 |     0.09 |
/// |   262144 |      1971 |      0.11 |      0.01 |     0.10 |
/// |   524288 |       986 |      0.12 |      0.02 |     0.09 |
///
/// ### Linux
///
/// > stat -f -c '%S' .
/// 4096
/// > uname -prsv
/// Linux 3.16.0-4-amd64 #1 SMP Debian 3.16.7-ckt25-2+deb8u3 (2016-07-02) unknown
///
/// | buffsize | num loops | real time | user time | sys time |
/// |----------|-----------|-----------|-----------|----------|
/// |        1 | 516581760 |    202.84 |     77.36 |   125.56 |
/// |        2 | 258290880 |    101.30 |     38.71 |    62.62 |
/// |        4 | 129145440 |     50.54 |     19.52 |    31.02 |
/// |        8 |  64572720 |     25.37 |      9.77 |    15.60 |
/// |       16 |  32286360 |     12.75 |      4.77 |     7.97 |
/// |       32 |  16143180 |      6.41 |      2.32 |     4.09 |
/// |       64 |   8071590 |      3.25 |      1.27 |     1.98 |
/// |      128 |   4035795 |      1.68 |      0.69 |     0.98 |
/// |      256 |   2017898 |      0.89 |      0.36 |     0.53 |
/// |      512 |   1008949 |      0.49 |      0.20 |     0.28 |
/// |     1024 |    504475 |      0.31 |      0.10 |     0.21 |
/// |     2048 |    252238 |      0.22 |      0.07 |     0.14 |
/// |     4096 |    126119 |      0.17 |      0.02 |     0.14 |
/// |     8192 |     63060 |      0.12 |      0.00 |     0.11 |
/// |    16384 |     31530 |      0.13 |      0.01 |     0.12 |
/// |    32768 |     15765 |      0.10 |      0.00 |     0.10 |
/// |    65536 |      7883 |      0.12 |      0.01 |     0.10 |
/// |   131072 |      3942 |      0.11 |      0.01 |     0.10 |
/// |   262144 |      1971 |      0.13 |      0.02 |     0.10 |
/// |   524288 |       986 |      0.13 |      0.04 |     0.09 |

extern crate libc;
#[macro_use(as_void)]
extern crate apue;
extern crate errno;

use libc::{STDIN_FILENO, STDOUT_FILENO, read, write};
use apue::LibcResult;
use errno::errno;
use std::io::Write;

fn main() {
    let args = std::env::args();
    let buffsize: usize = if args.len() == 2 {
        let args: Vec<String> = args.collect();
        args[1].parse::<usize>().expect("arg needs to be a number")
    } else {
        4096
    };
    unsafe {
        let mut num_loops = 0;
        let buf = vec![0; buffsize];
        while let Some(n) = read(STDIN_FILENO, as_void!(buf), buffsize).to_option() {
            assert!(write(STDOUT_FILENO, as_void!(buf), n as _) == n,
                    "write error");
            num_loops += 1;
        }
        if errno().0 > 0 {
            panic!("read error");
        }
        writeln!(&mut std::io::stderr(), "total loops: {:?}", num_loops).unwrap();
    }
}
