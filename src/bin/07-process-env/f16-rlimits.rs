/// Figure 7.16 Print the current resource limits
///
/// Takeaways:
///
/// - the nice trick of using the name of the constant plus the constant value
/// in one go (`#define doit(name)  pr_limits(#name, name)`) works also in Rust using macros and
/// `stringify!`
/// - coredumps on OSX are not activated by default, you need to activate them with
///   `ulimit -c unlimited` first: http://stackoverflow.com/questions/9412156
///
/// mac only:
/// $ f16-rlimits
/// RLIMIT_AS       (infinite)  (infinite)
/// RLIMIT_CORE              0  (infinite)
/// RLIMIT_CPU      (infinite)  (infinite)
/// RLIMIT_DATA     (infinite)  (infinite)
/// RLIMIT_FSIZE    (infinite)  (infinite)
/// RLIMIT_MEMLOCK  (infinite)  (infinite)
/// RLIMIT_NOFILE         2560  (infinite)
/// RLIMIT_NPROC           709        1064
/// RLIMIT_RSS      (infinite)  (infinite)
/// $ ulimit -c unlimited && f16-rlimits | grep CORE
/// RLIMIT_CORE     (infinite)  (infinite)
///
/// linux only:
/// $ f16-rlimits
/// RLIMIT_AS       (infinite)  (infinite)
/// RLIMIT_CORE              0  (infinite)
/// RLIMIT_CPU      (infinite)  (infinite)
/// RLIMIT_DATA     (infinite)  (infinite)
/// RLIMIT_FSIZE    (infinite)  (infinite)
/// RLIMIT_MEMLOCK       65536       65536
/// RLIMIT_MSGQUEUE     819200      819200
/// RLIMIT_NICE              0           0
/// RLIMIT_NOFILE         1024       65536
/// RLIMIT_NPROC         47643       47643
/// RLIMIT_RSS      (infinite)  (infinite)

extern crate libc;
extern crate apue;

use libc::{RLIM_INFINITY, RLIMIT_AS, RLIMIT_CORE, RLIMIT_CPU, RLIMIT_DATA, RLIMIT_FSIZE,
           RLIMIT_MEMLOCK, RLIMIT_NOFILE, RLIMIT_NPROC, RLIMIT_RSS};

#[cfg(target_os = "linux")]
use libc::{RLIMIT_MSGQUEUE, RLIMIT_NICE};

use libc::{rlimit, getrlimit};
use apue::LibcResult;

macro_rules! doit {
    ($s:expr) => {{
        pr_limits(stringify!($s), $s)
    }}
}

unsafe fn pr_limits(name: &str, resource: i32) {
    let mut limit: rlimit = std::mem::uninitialized();
    getrlimit(resource, &mut limit).check_not_negative().expect(&format!("getrlimit error for {}", name));
    print!("{:16}", name);
    match limit.rlim_cur {
        RLIM_INFINITY => print!("(infinite)  "),
        _ => print!("{:10}  ", limit.rlim_cur),
    };
    match limit.rlim_max {
        RLIM_INFINITY => print!("(infinite)"),
        _ => print!("{:10}", limit.rlim_max),
    };
    println!("");
}

fn main() {
    unsafe {
        doit!(RLIMIT_AS);
        doit!(RLIMIT_CORE);
        doit!(RLIMIT_CPU);
        doit!(RLIMIT_DATA);
        doit!(RLIMIT_FSIZE);
        doit!(RLIMIT_MEMLOCK);
        #[cfg(target_os = "linux")]
        doit!(RLIMIT_MSGQUEUE);
        #[cfg(target_os = "linux")]
        doit!(RLIMIT_NICE);
        doit!(RLIMIT_NOFILE);
        doit!(RLIMIT_NPROC);
        doit!(RLIMIT_RSS);
    }
}
