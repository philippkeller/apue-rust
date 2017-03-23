/// Figure 8.29 Print selected fields from system’s accounting file
///
/// Takeaway:
/// - on linux I first took acct instead of acct_v3, which had
///   "funny" results
/// - when doing dd to /dev/null the number of chars is 0, even though
///   the book says: "Even though the output goes to the null device, the
///   bytes are still accounted for", both on Linux as on MacOS
///
/// Things which could be improved in this code:
///
/// - use bindgen as macro directly in here instead of copy-pasting output
///   from bindgen commandline output
/// - maybe there's a better way to nul terminate string without doing
///   a copy of the string
///
// To try this script out:
//
// on linux:
// $ sudo accton /var/log/account/pacct
// .. do a few commands
// $ f29-acdata /var/log/account/pacct
//
// on mac:
// $ sudo touch /var/account/acct
// $ sudo accton /var/account/acct
// .. do a few commands
// $ f29-acdata /var/account/acct
// $ sudo accton # disable
#[macro_use(cstr)]
extern crate apue;
extern crate libc;

use libc::{fopen, fread, c_char, c_ushort, c_uint, c_void, ferror};
#[cfg(target_os = "macos")]
use libc::{c_uchar, c_int};

use apue::{LibcPtrResult, err_sys, array_to_string};

const AFORK: u8 = 0x01; // fork'd but not exec'd
const ASU: u8 = 0x02; // used super-user permissions
const ACORE: u8 = 0x08; // dumped core
const AXSIG: u8 = 0x10; // killed by a signal

#[cfg(target_os = "macos")]
const COMM_LEN: usize = 10;
#[cfg(target_os = "linux")]
const COMM_LEN: usize = 16;

// taken via bindgen from /usr/include/sys/acct.h
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
#[cfg(target_os = "macos")]
pub struct acct {
    pub ac_comm: [c_char; 10usize],
    pub ac_utime: c_ushort,
    pub ac_stime: c_ushort,
    pub ac_etime: c_ushort,
    pub ac_btime: c_uint,
    pub ac_uid: c_uint,
    pub ac_gid: c_uint,
    pub ac_mem: c_ushort,
    pub ac_io: c_ushort,
    pub ac_tty: c_int,
    pub ac_flag: c_uchar,
}

// taken via bindgen from /usr/include/linux/acct.h (acct_v3)
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
#[cfg(target_os = "linux")]
pub struct acct {
    pub ac_flag: c_char,
    pub ac_version: c_char,
    pub ac_tty: c_ushort,
    pub ac_exitcode: c_uint,
    pub ac_uid: c_uint,
    pub ac_gid: c_uint,
    pub ac_pid: c_uint,
    pub ac_ppid: c_uint,
    pub ac_btime: c_uint,
    pub ac_etime: f32,
    pub ac_utime: c_ushort,
    pub ac_stime: c_ushort,
    pub ac_mem: c_ushort,
    pub ac_io: c_ushort,
    pub ac_rw: c_ushort,
    pub ac_minflt: c_ushort,
    pub ac_majflt: c_ushort,
    pub ac_swaps: c_ushort,
    pub ac_comm: [c_char; 16usize],
}


fn flag2str(acdata: acct, flag: u8, char: &str) -> &str {
    if acdata.ac_flag as u8 & flag > 0 {
        char
    } else {
        " "
    }
}

fn main() {
    unsafe {
        let mut args = std::env::args();
        if args.len() != 2 {
            println!("usage: {} filename", args.next().unwrap());
            return;
        }
        let fname = args.next_back().unwrap();
        let fp = fopen(cstr!(fname.clone()), cstr!("r"))
            .check_not_null()
            .expect(&format!("can't open {}", fname));
        let mut acdata: acct = std::mem::zeroed();
        while fread(&mut acdata as *mut _ as *mut c_void,
                    std::mem::size_of::<acct>(),
                    1,
                    fp) == 1 {

            // manually nul terminate string
            let mut ac_comm = vec![0; COMM_LEN+1];
            ac_comm[..COMM_LEN].clone_from_slice(&acdata.ac_comm);

            println!("{:16} e = {:8}, chars = {:7}, {} {} {} {}",
                     // std::str::from_utf8(ac_comm),
                     array_to_string(&ac_comm),
                     acdata.ac_etime,
                     acdata.ac_io,
                     flag2str(acdata, ACORE, "D"),
                     flag2str(acdata, AXSIG, "X"),
                     flag2str(acdata, AFORK, "F"),
                     flag2str(acdata, ASU, "S"))
        }
        if ferror(fp) != 0 {
            err_sys("read error");
        }
    }
}