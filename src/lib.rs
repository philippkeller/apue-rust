extern crate libc;
extern crate errno;
extern crate num;

use libc::{c_int, c_char, dev_t, utsname, sigset_t, sighandler_t, PATH_MAX, SA_RESTART, EINTR};
use libc::{SIG_ERR, SIG_BLOCK, SIG_IGN, SIG_SETMASK, SIGALRM, SIGINT, SIGUSR1, SIGQUIT, SIGCHLD};
use libc::{WSTOPSIG, WEXITSTATUS, WIFSTOPPED, WCOREDUMP, WTERMSIG, WIFSIGNALED, WIFEXITED};
use libc::{exit, _exit, sigemptyset, sigaddset, sigaction, sigismember, fork, waitpid};
use my_libc::{sigprocmask, execl, sys_nerr, sys_errlist};
use std::io::Write;
use std::ffi::CStr;
use std::mem::{zeroed, uninitialized};
use std::ptr::{null, null_mut};
use std::io::{Result, Error};
use num::{Num, Zero, One, Signed};

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
    #[deprecated(since="0.0.2", note="please use `check_*` instead")]
    fn to_option(&self) -> Option<T>;
    fn check_not_negative(&self) -> Result<T>;
    fn check_positive(&self) -> Result<T>;
    fn check_minus_one(&self) -> Result<T>;
}

pub trait LibcPtrResult<T> {
    fn check_not_null(&self) -> Result<T>;
}

fn check_positive<N: num::Num + PartialOrd + Copy>(val:N) -> Result<N> {
    if val < N::zero() {
        Err(Error::last_os_error())
    } else if val == N::zero() {
        Err(Error::from_raw_os_error(0))
    } else {
        Ok(val)
    }
}
fn check_not_negative<N: num::Num + PartialOrd + Copy>(val:N) -> Result<N> {
    if val < N::zero() {
        Err(Error::last_os_error())
    } else {
        Ok(val)
    }
}

fn check_minus_one<N: num::Num + PartialOrd + Copy + Signed>(val:N) -> Result<N> {
    if val.is_negative() && val.abs() == N::one() {
        Err(Error::from(std::io::ErrorKind::Other))
    } else {
        Ok(val)
    }
}

impl LibcResult<c_int> for c_int {
    fn to_option(&self) -> Option<c_int> {
        if *self < 0 { None } else { Some(*self) }
    }
    fn check_not_negative(&self) -> Result<c_int> {
        check_not_negative(*self)
    }
    fn check_positive(&self) -> Result<c_int> {
        check_positive(*self)
    }
    fn check_minus_one(&self) -> Result<c_int> {
        check_minus_one(*self)
    }
}
impl LibcResult<i64> for i64 {
    fn to_option(&self) -> Option<i64> {
        if *self < 0 { None } else { Some(*self) }
    }
    fn check_not_negative(&self) -> Result<i64> {
        check_not_negative(*self)
    }
    fn check_positive(&self) -> Result<i64> {
        check_positive(*self)
    }
    fn check_minus_one(&self) -> Result<i64> {
        check_minus_one(*self)
    }
}

impl LibcResult<isize> for isize {
    fn to_option(&self) -> Option<isize> {
        if *self <= 0 { None } else { Some(*self) }
    }
    fn check_not_negative(&self) -> Result<isize> {
        check_not_negative(*self)
    }
    fn check_positive(&self) -> Result<isize> {
        check_positive(*self)
    }
    fn check_minus_one(&self) -> Result<isize> {
        check_minus_one(*self)
    }
}

impl LibcResult<usize> for usize {
    fn to_option(&self) -> Option<usize> {
        if *self == SIG_ERR { None } else { Some(*self) }
    }
    fn check_not_negative(&self) -> Result<usize> {
        check_not_negative(*self)
    }
    fn check_positive(&self) -> Result<usize> {
        check_positive(*self)
    }
    fn check_minus_one(&self) -> Result<usize> {
        // usize is unsigned -> can never be minus one
        Err(Error::from(std::io::ErrorKind::Other))
    }
}

// legacy, will go away
impl<T> LibcResult<*mut T> for *mut T {
    fn to_option(&self) -> Option<*mut T> {
        if self.is_null() { None } else { Some(*self) }
    }
    fn check_not_negative(&self) -> Result<*mut T> {
        unimplemented!()
    }
    fn check_positive(&self) -> Result<*mut T> {
        unimplemented!();
    }
    fn check_minus_one(&self) -> Result<*mut T> {
        unimplemented!();
    }
}

impl<T> LibcPtrResult<*mut T> for *mut T {
    fn check_not_null(&self) -> Result<*mut T> {
        if self.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(*self)
        }
    }
}

pub trait PthreadExpect {
    fn expect(&self, msg: &str);
}

impl PthreadExpect for c_int {
    fn expect(&self, msg: &str) {
        if *self != 0 {
            println!("{}: error {}", msg, *self);
            unsafe {
                exit(1);
            }
        }
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

pub fn strerror(error: i32) -> String {
    unsafe {
        if error >= 0 && error <= sys_nerr {
            // avoiding rusts array bound checking (sys_errlist has length 0usize as the length
            // is not known at compile time)
            CStr::from_ptr(*sys_errlist.as_ptr().offset(error as _) as _)
                .to_owned()
                .into_string()
                .expect("not valid string")
        } else {
            format!("Unknown error {}", error)
        }
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
        sigprocmask(0, null(), &mut sigset).check_not_negative().expect("sigprocmask error");
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

// Figure 8.22 The system function, without signal handling
pub unsafe fn system(cmdstring: &str) -> Option<i32> {
    if let Some(pid) = fork().to_option() {
        match pid {
            0 => {
                // child
                execl(cstr!("/bin/sh"),
                      cstr!("sh"),
                      cstr!("-c"),
                      cstr!(cmdstring),
                      0 as *const c_char);
                _exit(127);
            }
            _ => {
                // parent
                let mut status = 0;
                while waitpid(pid, &mut status, 0) < 0 {
                    if errno::errno().0 != EINTR {
                        return None;
                    }
                }
                return Some(status);
            }
        }
    } else {
        return None;
    }
}


// Figure 10.28 Correct POSIX.1 implementation of system function
// (with signal handling)
pub unsafe fn system2(cmdstring: &str) -> std::result::Result<i32, String> {
    let mut ignore: sigaction = std::mem::zeroed();
    let (mut saveintr, mut savemask, mut savequit) = uninitialized();
    ignore.sa_sigaction = SIG_IGN; // ignore SIGINT and SIGQUIT
    sigemptyset(&mut ignore.sa_mask);
    ignore.sa_flags = 0;
    sigaction(SIGINT, &ignore, &mut saveintr).to_option().ok_or("sigaction error")?;
    sigaction(SIGQUIT, &ignore, &mut savequit).to_option().ok_or("sigaction error")?;
    let mut chldmask = uninitialized();
    sigemptyset(&mut chldmask);
    sigaddset(&mut chldmask, SIGCHLD);
    sigprocmask(SIG_BLOCK, &chldmask, &mut savemask).to_option().ok_or("sigprocmask error")?;

    let pid = fork().to_option().ok_or("fork error")?;
    let mut status = 0;
    if pid == 0 {
        sigaction(SIGINT, &mut saveintr, null_mut());
        sigaction(SIGQUIT, &mut savequit, null_mut());
        sigprocmask(SIG_SETMASK, &mut savemask, null_mut());
        execl(cstr!("/bin/sh"),
              cstr!("sh"),
              cstr!("-c"),
              cstr!(cmdstring),
              0 as *const c_char);
        _exit(127); // exec error
    } else {
        while waitpid(pid, &mut status, 0) < 0 {
            if errno::errno().0 != EINTR {
                return Err(format!("waitpid error, got error {:?}", errno::errno()));
            }
        }
    }
    sigaction(SIGINT, &saveintr, null_mut()).to_option().ok_or("sigaction error")?;
    sigaction(SIGQUIT, &savequit, null_mut()).to_option().ok_or("sigaction error")?;
    sigprocmask(SIG_SETMASK, &savemask, null_mut()).to_option().ok_or("sigprocmask error")?;
    Ok(status)
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


/// Figure 10.24 Routines to allow a parent and child to synchronize
///
/// Status: only compiles, did not yet run it to check for correctness
/// has still bugs for sure..
pub mod sync_parent_child {
    use my_libc::{sigprocmask, sigsuspend};
    use LibcResult;
    use libc::{SIGUSR1, SIGUSR2, SIG_BLOCK, SIG_SETMASK, c_int, pid_t, sigset_t};
    use libc::{signal, sigemptyset, sigaddset, kill};
    use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
    use std::mem::uninitialized;
    use std::ptr::null_mut;

    static mut SIGFLAG: AtomicBool = ATOMIC_BOOL_INIT;
    //    #[cfg(target_os = "macos")]
    //    static mut OLDMASK: sigset_t = 0;
    //    #[cfg(target_os = "macos")]
    //    static mut ZEROMASK: sigset_t = 0;
    //    #[cfg(target_os = "linux")]
    //    static mut OLDMASK: sigset_t = sigset_t { __val: [0;16] };
    //    #[cfg(target_os = "linux")]
    //    static mut ZEROMASK: sigset_t = sigset_t { __val: [0;16] };

    pub fn sig_usr(_: c_int) {
        unsafe {
            SIGFLAG.store(true, Ordering::SeqCst);
        }
    }

    pub fn tell_wait() -> sigset_t {
        unsafe {
            let mut newmask = uninitialized();
            signal(SIGUSR1, sig_usr as usize).to_option().expect("signal(SIGUSR1) error");
            signal(SIGUSR2, sig_usr as usize).to_option().expect("signal(SIGUSR2) error");
            let mut zeromask = uninitialized();
            sigemptyset(&mut zeromask);
            sigemptyset(&mut newmask);
            sigaddset(&mut newmask, SIGUSR1);
            sigaddset(&mut newmask, SIGUSR2);

            // Block SIGUSR1 and SIGUSR2 and save current signal mask
            let mut oldmask = uninitialized();
            sigprocmask(SIG_BLOCK, &newmask, &mut oldmask).to_option().expect("SIG_BLOCK error");
            oldmask
        }
    }

    pub unsafe fn tell_parent(pid: pid_t) {
        kill(pid, SIGUSR2); // tell parent we're done
    }

    pub unsafe fn wait_parent(oldmask: sigset_t) {
        let mut zeromask = uninitialized();
        sigemptyset(&mut zeromask);
        // run until sigflag becomes true, then set it to false again immediately
        while !SIGFLAG.compare_and_swap(true, false, Ordering::SeqCst) {
            sigsuspend(&zeromask);
        }
        // Reset signal mask to original value
        sigprocmask(SIG_SETMASK, &oldmask, null_mut()).to_option().expect("SIG_SETMASK error");
    }

    pub unsafe fn tell_child(pid: pid_t) {
        kill(pid, SIGUSR1); // tell child we’re done
    }

    pub unsafe fn wait_child(oldmask: sigset_t) {
        let mut zeromask = uninitialized();
        sigemptyset(&mut zeromask);
        // run until sigflag becomes true, then set it to false again immediately
        while !SIGFLAG.compare_and_swap(true, false, Ordering::SeqCst) {
            sigsuspend(&zeromask);
        }
        // Reset signal mask to original value
        sigprocmask(SIG_SETMASK, &oldmask, null_mut())
            .to_option()
            .expect("SIG_SETMASK error");
    }
}

#[allow(non_camel_case_types)]
pub mod my_libc {
    use libc::{dirent, c_void, c_int, c_char, c_long, c_ulong, c_uint, pid_t, clock_t, siginfo_t,
               sigset_t, id_t, size_t, tm, pthread_attr_t, pthread_t, pthread_rwlock_t};
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

    #[repr(C)]
    #[derive(Copy, Clone)]
    #[derive(Debug)]
    #[cfg(target_os = "macos")]
    pub struct pthread_rwlockattr_t {
        pub __sig: ::std::os::raw::c_long,
        pub __opaque: [::std::os::raw::c_char; 16usize],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    #[derive(Debug)]
    #[cfg(target_os = "linux")]
    pub struct pthread_rwlockattr_t {
        pub _bindgen_data_: [u64; 1usize],
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

        #[cfg(target_os = "macos")]
        #[link_name = "__stdinp"]
        pub static mut stdin: *mut FILE;

        #[cfg(not(target_os = "macos"))]
        pub static mut stdin: *mut FILE;

        pub fn times(arg1: *mut tms) -> clock_t;

        pub fn sigprocmask(arg1: c_int, arg2: *const sigset_t, arg3: *mut sigset_t) -> c_int;
        pub fn sigpending(arg1: *mut sigset_t) -> c_int;
        pub fn sigsuspend(arg1: *const sigset_t) -> c_int;

        pub fn strftime(s: *mut c_char,
                        maxsize: size_t,
                        format: *const c_char,
                        timeptr: *const tm)
                        -> size_t;

        pub fn pthread_exit(arg1: *mut c_void);
        pub fn pthread_create(arg1: *mut pthread_t,
                              arg2: *const pthread_attr_t,
                              arg3: unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void,
                              arg4: *mut c_void)
                              -> c_int;
        pub fn pthread_rwlock_init(arg1: *mut pthread_rwlock_t,
                                   arg2: *const pthread_rwlockattr_t)
                                   -> c_int;

        pub static mut sys_errlist: [*const c_char; 0usize];
        pub static sys_nerr: c_int;

        pub fn qsort(__base: *mut c_void,
                     __nel: size_t,
                     __width: size_t,
                     __comp: unsafe extern "C" fn(a1: *const c_void, a2: *const c_void) -> c_int)
                     -> c_int;
        #[cfg(target_os = "macos")]
        pub fn heapsort(__base: *mut c_void,
                        __nel: size_t,
                        __width: size_t,
                        __comp: unsafe extern "C" fn(a1: *const c_void, a2: *const c_void) -> c_int)
                        -> c_int;
        pub fn random() -> c_long;
        pub fn srandom(arg1: c_uint);
    }
}
