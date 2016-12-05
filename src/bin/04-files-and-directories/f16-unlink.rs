/// Figure 4.16 Open a file and then unlink it
///
/// In the example in the book the program waits for 15 seconds
/// omitted that part as it slows down the test.py tests
///
/// $ touch /tmp/f16-unlink.tmp
/// $ f16-unlink
///
/// mac only:
/// $ ls /tmp/f16-unlink.tmp 2>&1
/// ls: /tmp/f16-unlink.tmp: No such file or directory
/// ERROR: return code 1
///
/// linux only:
/// $ ls /tmp/f16-unlink.tmp 2>&1
/// ls: cannot access '/tmp/f16-unlink.tmp': No such file or directory
/// ERROR: return code 2

// proof that this works (only in // so it is not executed by test.py)
//
// $ dd if=/dev/zero of=/tmp/f16-unlink.tmp bs=999999 count=9999 2>/dev/null
// $ df -h /tmp/
// Filesystem   Size   Used  Avail Capacity iused      ifree %iused  Mounted on
// /dev/disk1  931Gi  398Gi  533Gi    43% 3467205 4291500074    0%   /
//
// $ f16-unlink &
// [1] 92883
//
// $ ls /tmp/f16-unlink.tmp
// ls: /tmp/f16-unlink.tmp: No such file or directory
//
// $ df -kh /tmp/
// Filesystem   Size   Used  Avail Capacity iused      ifree %iused  Mounted on
// /dev/disk1  931Gi  398Gi  533Gi    43% 3467369 4291499910    0%   /
//
// # after 15 seconds..
// $ file unlinked
// [1]+  Done                    f16-unlink
//
// $ df -h /tmp/
// Filesystem   Size   Used  Avail Capacity iused      ifree %iused  Mounted on
// /dev/disk1  931Gi  388Gi  542Gi    42% 3467369 4291499910    0%   /

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{open, sleep, unlink, c_int};
use apue::{LibcResult, err_sys};

const O_RDWR: c_int = 2; // taken from /usr/include/sys/fcntl.h

fn main() {
    unsafe {
        if let None = open(cstr!("/tmp/f16-unlink.tmp"), O_RDWR).to_option() {
            err_sys("open error");
        }

        if let None = unlink(cstr!("/tmp/f16-unlink.tmp")).to_option() {
            err_sys("unlink error");
        }
        sleep(0);// change this to 15 to make the test as explained in the book
        println!("file unlinked");
    }
}
