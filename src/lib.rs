extern crate libc;
extern crate errno;

use libc::{c_int, c_char, dev_t, utsname, sigset_t, sighandler_t, PATH_MAX, SA_RESTART};
use libc::{SIG_ERR, SIGALRM, SIGINT, SIGUSR1, SIGQUIT};
use libc::{WSTOPSIG, WEXITSTATUS, WIFSTOPPED, WCOREDUMP, WTERMSIG, WIFSIGNALED, WIFEXITED};
use libc::{exit, sigemptyset, sigaction, sigismember};
use my_libc::sigprocmask;
use std::io::Write;
use std::ffi::CStr;
use std::mem::{zeroed, uninitialized};

/// Turns a str into a c string. Warning: the cstring only lives as long the
/// str lives. Don't e.g. assign the return value to a variable!
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {{
        use std::ffi::CString;
        CString::new($s).unwrap().as_ptr()
    }}
}

#[macro_export]
macro_rules! as_void {
    ($s:expr) => {{
        extern crate libc;
        use libc::c_void;
        $s.as_ptr() as *mut c_void
    }}
}

#[macro_export]
macro_rules! as_char {
    ($s:expr) => {{
        extern crate libc;
        use libc::c_char;
        $s.as_ptr() as *mut c_char
    }}
}

#[macro_export]
macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;
            if let Err(e) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
                panic!("Failed to write to stderr.\
                    \nOriginal error output: {}\
                    \nSecondary error writing to stderr: {}", format!($($arg)*), e);
            }
        }
    )
}

/// turn libc result into an option
pub trait LibcResult<T> {
    /// returns None if the result is empty (-1 if an integer, Null if a pointer)
    /// and Some otherwise
    ///
    /// # Example
    /// if let Some(fd) = libc::creat(fd1, FILE_MODE).to_option() {
    ///     fd
    /// } else {
    ///     panic!("{}", io::Error::last_os_error());
    /// }
    fn to_option(&self) -> Option<T>;
}

impl LibcResult<c_int> for c_int {
    fn to_option(&self) -> Option<c_int> {
        if *self < 0 { None } else { Some(*self) }
    }
}
impl LibcResult<i64> for i64 {
    fn to_option(&self) -> Option<i64> {
        if *self < 0 { None } else { Some(*self) }
    }
}

// implementation for isize, sentinel = 0 (means end of file/buffer/... e.g. in read)
impl LibcResult<isize> for isize {
    fn to_option(&self) -> Option<isize> {
        if *self <= 0 { None } else { Some(*self) }
    }
}

impl LibcResult<sighandler_t> for sighandler_t {
    fn to_option(&self) -> Option<sighandler_t> {
        if *self == SIG_ERR { None } else { Some(*self) }
    }
}


impl<T> LibcResult<*mut T> for *mut T {
    fn to_option(&self) -> Option<*mut T> {
        if self.is_null() { None } else { Some(*self) }
    }
}

pub unsafe fn array_to_string(sl: &[i8]) -> &str {
    CStr::from_ptr(sl.as_ptr()).to_str().expect("invalid string")
}

/// Return uname -s
pub fn uname() -> Option<String> {
    let mut uc: utsname = unsafe { uninitialized() };
    unsafe {
        if libc::uname(&mut uc) == 0 {
            return Some(String::from(array_to_string(&uc.sysname)));
        }
    }
    None
}

pub fn err_sys(msg: &str) {
    std::io::stderr().write(format!("{}{}", msg, "\n").as_bytes()).unwrap();
    unsafe {
        exit(1);
    }
}

pub fn path_alloc() -> std::vec::Vec<c_char> {
    // Before POSIX.1-2001, we aren’t guaranteed that PATH_MAX includes
    // the terminating null byte. For simplicity sake we don't check for the posix
    // version and just increase by one
    Vec::with_capacity((PATH_MAX + 1) as usize)
}
// major device number, impl ported from /usr/include/sys/types.h
pub fn major(x: dev_t) -> dev_t {
    (x >> 24) & 0xff
}

// minor device number, impl ported from /usr/include/sys/types.h
pub fn minor(x: dev_t) -> dev_t {
    x & 0xffffff
}

pub fn pr_exit(status: c_int) {
    unsafe {
        if WIFEXITED(status) {
            println!("normal termination, exit status = {}", WEXITSTATUS(status));
        } else if WIFSIGNALED(status) {
            println!("abnormal termination, signal number = {} {}",
                     WTERMSIG(status),
                     if WCOREDUMP(status) {
                         " (core file generated)"
                     } else {
                         ""
                     });
        } else if WIFSTOPPED(status) {
            println!("child stopped, signal number = {}", WSTOPSIG(status));
        }
    }
}

macro_rules! print_sig {
    ($set:expr, $s:expr) => {{
        if sigismember($set, $s) == 1 {
            print!(" {}", stringify!($s));
        }
    }}
}

pub fn pr_mask(s: &str) {
    unsafe {
        let errno_save = errno::errno();
        let mut sigset: sigset_t = std::mem::uninitialized();
        sigprocmask(0, std::ptr::null(), &mut sigset).to_option().expect("sigprocmask error");
        print!("{}", s);
        print_sig!(&sigset, SIGINT);
        print_sig!(&sigset, SIGQUIT);
        print_sig!(&sigset, SIGUSR1);
        print_sig!(&sigset, SIGALRM);
        print!("\n");
        errno::set_errno(errno_save);
    }
}

// Figure 10.18: Reliable version of signal(), using POSIX sigaction()
pub unsafe fn signal(signo: i32, func: fn(c_int)) -> usize {
    let mut act: sigaction = zeroed();
    let mut oact: sigaction = uninitialized();
    act.sa_sigaction = func as usize;
    sigemptyset(&mut act.sa_mask);
    act.sa_flags = 0;
    if signo != SIGALRM {
        act.sa_flags |= SA_RESTART;
    }
    if sigaction(signo, &act, &mut oact) < 0 {
        SIG_ERR
    } else {
        oact.sa_sigaction as usize
    }
}

// Figure 10.19: The signal_intr function, same as signal() above
// with the only difference that no system call is restarted
pub unsafe fn signal_intr(signo: i32, func: fn(c_int)) -> usize {
    let mut act: sigaction = zeroed();
    let mut oact: sigaction = uninitialized();
    act.sa_sigaction = func as usize;
    sigemptyset(&mut act.sa_mask);
    act.sa_flags = 0;
    if sigaction(signo, &act, &mut oact) < 0 {
        SIG_ERR
    } else {
        oact.sa_sigaction as usize
    }
}

#[allow(non_camel_case_types)]
pub mod my_libc {
    use libc::{dirent, c_int, c_char, c_long, c_ulong, pid_t, clock_t, siginfo_t, sigset_t, id_t};
    use libc::{DIR, FILE};

    #[repr(C)]
    #[derive(Copy, Clone)]
    #[derive(Debug)]
    pub struct spwd {
        pub sp_namp: *mut c_char,
        pub sp_pwdp: *mut c_char,
        pub sp_lstchg: c_long,
        pub sp_min: c_long,
        pub sp_max: c_long,
        pub sp_warn: c_long,
        pub sp_inact: c_long,
        pub sp_expire: c_long,
        pub sp_flag: c_ulong,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    #[derive(Debug)]
    pub struct tms {
        pub tms_utime: clock_t,
        pub tms_stime: clock_t,
        pub tms_cutime: clock_t,
        pub tms_cstime: clock_t,
    }

    #[derive(Copy, Clone)]
    #[repr(u32)]
    #[derive(Debug)]
    pub enum idtype_t {
        P_ALL = 0,
        P_PID = 1,
        P_PGID = 2,
    }

    pub const WEXITED: c_int = 0x00000004;  // [XSI] Processes which have exitted
    pub const WSTOPPED: c_int = 0x00000008;  // [XSI] Any child stopped by signal
    pub const WCONTINUED: c_int = 0x00000010;  // [XSI] Any child stopped then continued
    pub const WNOWAIT: c_int = 0x00000020;  // [XSI] Leave process returned waitable

    pub const CLD_NOOP: c_int = 0;       // if only I knew...
    pub const CLD_EXITED: c_int = 1;       // [XSI] child has exited
    pub const CLD_KILLED: c_int = 2;       // [XSI] terminated abnormally, no core file
    pub const CLD_DUMPED: c_int = 3;       // [XSI] terminated abnormally, core file
    pub const CLD_TRAPPED: c_int = 4;       // [XSI] traced child has trapped
    pub const CLD_STOPPED: c_int = 5;       // [XSI] child has stopped
    pub const CLD_CONTINUED: c_int = 6;       // [XSI] stopped child has continued

    extern "C" {
        #[cfg(target_os = "macos")]
        #[link_name = "readdir$INODE64"]
        pub fn readdir(arg1: *mut DIR) -> *mut dirent;

        #[cfg(not(target_os = "macos"))]
        pub fn readdir(arg1: *mut DIR) -> *mut dirent;

        pub fn dirfd(dirp: *mut DIR) -> c_int;

        pub fn tmpnam(ptr: *mut c_char) -> *mut c_char;

        pub fn getc(arg1: *mut FILE) -> c_int;
        pub fn putc(arg1: c_int, arg2: *mut FILE) -> c_int;
        pub fn getchar() -> c_int;

        pub fn setspent();
        pub fn endspent();
        pub fn getspent() -> *mut spwd;
        pub fn getspnam(__name: *const c_char) -> *mut spwd;

        // vfork is not implemented in libc, and that's probably good so
        // as vfork is somehow deprecated
        pub fn vfork() -> pid_t;

        pub fn execl(__path: *const c_char, __arg0: *const c_char, ...) -> c_int;
        pub fn execle(__path: *const c_char, __arg0: *const c_char, ...) -> c_int;
        pub fn execlp(__file: *const c_char, __arg0: *const c_char, ...) -> c_int;

        pub fn waitid(arg1: idtype_t, arg2: id_t, arg3: *mut siginfo_t, arg4: c_int) -> c_int;

        #[cfg(target_os = "macos")]
        #[link_name = "__stdoutp"]
        pub static mut stdout: *mut FILE;

        #[cfg(not(target_os = "macos"))]
        pub static mut stdout: *mut FILE;

        pub fn times(arg1: *mut tms) -> clock_t;

        pub fn sigprocmask(arg1: c_int, arg2: *const sigset_t, arg3: *mut sigset_t) -> c_int;
        pub fn sigpending(arg1: *mut sigset_t) -> c_int;
        pub fn sigsuspend(arg1: *const sigset_t) -> c_int;
    }
}
