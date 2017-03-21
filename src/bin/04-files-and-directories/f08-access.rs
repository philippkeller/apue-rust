/// Figure 4.8: Example of access function
///
/// Nothing special here.. only routine work
///
/// $ f08-access /etc/passwd
/// read access OK
/// open for reading OK
///
/// mac only:
/// $ f08-access /etc/master.passwd
/// access error for "/etc/master.passwd"
/// ERROR: return code 1
///
/// linux only:
/// $ f08-access /etc/shadow
/// access error for "/etc/shadow"
/// ERROR: return code 1


extern crate clap;
extern crate libc;
extern crate apue;

use clap::App;
use libc::{R_OK, O_RDONLY, access, open, exit};
use std::ffi::CString;
use apue::LibcResult;

fn main() {
    let matches = App::new("check access rights").args_from_usage("<filename>").get_matches();
    let filename = CString::new(matches.value_of("filename").unwrap()).unwrap();
    unsafe {
        if access(filename.as_ptr(), R_OK).check_not_negative().is_err() {
            println!("access error for {:?}", filename);
            exit(1);
        } else {
            println!("read access OK");
        }
        if open(filename.as_ptr(), O_RDONLY).check_not_negative().is_err() {
            println!("open error for {:?}", filename);
            exit(1);
        } else {
            println!("open for reading OK");
        }
    }
}
