/// Figure 8.29 Print selected fields from system’s accounting file
///
/// Status: compiles and runs on MacOs, didn't yet check if the output is correct
/// (at least ac_comm looks good)

#[macro_use(cstr)]
extern crate apue;
extern crate libc;

use libc::{fopen, fread, c_char, c_ushort, c_uchar, c_uint, c_int, c_void, ferror};
use apue::{LibcResult, err_sys, array_to_string};

const AFORK: u8 = 0x01; // fork'd but not exec'd
const ASU: u8 = 0x02; // used super-user permissions
const ACORE: u8 = 0x08; // dumped core
const AXSIG: u8 = 0x10; // killed by a signal

// taken via bindgen from /usr/include/sys/acct.h
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
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

fn flag2str(acdata: acct, flag: u8, char: &str) -> &str {
    if acdata.ac_flag & flag > 0 { char } else { " " }
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
            .to_option()
            .expect(&format!("can't open {}", fname));
        let mut acdata: acct = std::mem::zeroed();
        while fread(&mut acdata as *mut _ as *mut c_void,
              std::mem::size_of::<acct>(),
              1,
              fp) == 1 {
            acdata.ac_comm[9] = 0; // 0-terminate string, otherwise: buffer-over-read
            println!("{:10} e = {:8} chars = {:8} {} {} {} {}",
                     array_to_string(&acdata.ac_comm),
                     acdata.ac_etime,
                     acdata.ac_io,
                     flag2str(acdata, ACORE, "D"),
                     flag2str(acdata, AXSIG, "X"),
                     flag2str(acdata, AFORK, "F"),
                     flag2str(acdata, ASU, "S"));
        }
        if ferror(fp) != 0 {
            err_sys("read error");
        }
    }
}