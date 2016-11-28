/// Code for Figure 5.13 (Demonstrate mkstemp function)
///
/// implementation in rust was quite straight forward
/// only caveat was that the pointer to the variable on the stack
/// is not possible (or I just didn't find out how to do that and
/// didn't dare to ask on stackoverflow)
///
/// $ f13-mkstemp 2>/dev/null
/// trying to create first temp file...
/// file exists

extern crate libc;
#[macro_use(print_err)]
extern crate apue;

use std::ffi::{CString, CStr};
use std::io;
use std::mem;

fn make_temp(template: *mut libc::c_char) -> Result<String, String> {
    unsafe {
        let fd = libc::mkstemp(template);
        if fd < 0 {
            return Err("can't create tmp dir".to_owned());
        }
        print_err!("temp name = {:?}", CStr::from_ptr(template));
        libc::close(fd);
        let mut stat: libc::stat = mem::uninitialized();
        if libc::stat(template, &mut stat) < 0 {
            if io::Error::last_os_error().raw_os_error().unwrap() == libc::ENOENT {
                println!("file doesn’t exist");
            } else {
                return Err("stat failed".to_owned());
            }
        } else {
            println!("file exists");
            libc::unlink(template);
        }
        Ok(CString::from_raw(template).into_string().unwrap())
    }
}

fn main() {
    let good_template = CString::new("/tmp/dirXXXXXX").unwrap();
    println!("trying to create first temp file...");
    let res = make_temp(good_template.into_raw());
    print_err!("{:?}", res);

    // the second part with the bad template I was just
    // unable to do in rust, seems that the type safety
    // was good enough that even after 30 minutes I couldn't
    // succeed
}
