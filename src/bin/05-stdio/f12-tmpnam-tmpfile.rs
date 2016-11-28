/// Code for Figure 5.12 (Demonstrate tmpnam and tmpfile functions)
///
/// Sad thing is that the rust code is about double the number of lines (40 lines)
/// of the C code (20 lines), mostly because of the string conversions.
/// It's a bit silly that there's no neat abstraction for that.
///
/// $ f12-tmpnam-tmpfile 2>/dev/null
/// one line of output

extern crate libc;
#[macro_use(print_err)]
extern crate apue;

use std::ffi::{CStr, CString};

const MAXLINE: usize = 4096;

extern "C" {
    pub fn tmpnam(ptr: *mut libc::c_char) -> *mut libc::c_char;
}

fn main() {
    // method 1: get pointer to new buffer
    let tmp = unsafe {
        let tmp_ptr = tmpnam(std::ptr::null_mut());
        CStr::from_ptr(tmp_ptr).to_owned().into_string().unwrap()
    };
    print_err!("tmp file={}", tmp);

    // method 2: create buffer ourselves, make tmpnam fill this buffer
    let tmp = unsafe {
        let name = CString::from_vec_unchecked(Vec::with_capacity(libc::L_tmpnam as usize))
            .into_raw();
        tmpnam(name);
        CStr::from_ptr(name).to_owned().into_string().unwrap()
    };
    print_err!("tmp file={}", tmp);

    unsafe {
        let fp = libc::tmpfile();
        if fp.is_null() {
            panic!("tmpfile error");
        }
        libc::fputs(CString::new("one line of output").unwrap().as_ptr(), fp);
        libc::rewind(fp);
        let line = CString::from_vec_unchecked(Vec::with_capacity(MAXLINE)).into_raw();
        if libc::fgets(line, MAXLINE as i32, fp).is_null() {
            panic!("fgets error");
        }
        println!("{}", CStr::from_ptr(line).to_owned().into_string().unwrap());
    }
}