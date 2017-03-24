/// Figure 8.30 Evaluate the effect of changing the nice value
///
/// Takeaway: running this "as is" on a modern laptop doesn't yield
/// any results as the forks run on different cpu cores and unless
/// all cores are not idle the parent and child get a "full core"
/// and hence there's no real difference in the count on child/parent.
///
/// The relevant value here is the number of "logical cores" which
/// is the number of cores times the numbers of "threads per core",
/// (hyperthreading)
/// see http://unix.stackexchange.com/a/88290/168663
/// OSX: `sysctl -n hw.ncpu`
/// linux: `lscpu` - logical cores = CPU(s) x Core(s) per socket x Thread(s) per core
///
/// To let the script run in parallel do e.g. this:
/// for i in {1..4}; do f30-nice 20 &; done
/// On OSX I needed to go about 2x the number of logical cores
/// to really see a difference. In Linux on virtualbox the effect
/// was already big (factor 10) when running at 1 x the number of logical cores

extern crate libc;
extern crate apue;
extern crate errno;

use libc::{gettimeofday, timeval, exit, setbuf, fork, nice};
use apue::{LibcResult, err_sys};
use apue::my_libc::stdout;
use std::ptr::null_mut as null;
use errno::{errno, Errno, set_errno};

unsafe fn checktime(end: timeval, str: &str, count: u64) {
    let mut tv: timeval = std::mem::uninitialized();
    gettimeofday(&mut tv, null());
    if tv.tv_sec >= end.tv_sec && tv.tv_usec >= end.tv_usec {
        println!("{} count = {}", str, count);
        exit(0);
    }
}

fn main() {
    unsafe {
        setbuf(stdout, null());
        // cannot access DEFINEs in rust and _SC_NZERO is not defined in Macos.
        // it's 20 anyway..
        let nzero = 20;
        let adj = if std::env::args().len() == 2 {
            std::env::args().next_back().unwrap().parse::<i32>().expect("not a number")
        } else {
            0
        };
        let mut end: timeval = std::mem::uninitialized();
        gettimeofday(&mut end, null());
        end.tv_sec += 10;
        let pid = fork().check_not_negative().expect("fork error");
        let str = if pid == 0 {
            println!("current nice value in child is {}, adjusting by {}",
                     nice(0) + nzero,
                     adj);
            set_errno(Errno(0));
            if nice(adj) == -1 && errno().0 != 0 {
                err_sys("child set scheduling priority");
            }
            println!("now child nice value is {}", nice(0) + nzero);
            "child"
        } else {
            println!("current nice value in parent is {}", nice(0) + nzero);
            "parent"
        };
        let mut count: u64 = 0;
        loop {
            count += 1; // panics when overflowing in debug mode
            checktime(end, str, count);
        }
    }
}
