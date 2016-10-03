/// Figure 1.3 List all the files in a directory
///
/// Takeaways: Strangely the libc binding on os x
/// was wrong and it took me like 2 hours to figure
/// out what is wrong (as bindgen also gave the same
/// binding as libc). In the end, the freebsd binding
/// worked. See [issue on rust-lang](https://github.com/rust-lang/libc/issues/414)

extern crate libc;

use std::ffi::{CString, CStr};
use std::str;
use libc::{opendir, closedir, DIR, dirent, c_char};

extern "C" {
    pub fn readdir(arg1: *mut DIR) -> *mut dirent;
}

pub struct MyDirent {
    pub d_fileno: u32,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_namlen: u8,
    pub d_name: [c_char; 256],
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: {:?} /my/path", args[0]);
        return;
    }
    let dir = CString::new(args[1].clone()).unwrap();
    unsafe {
        let dp = opendir(dir.as_ptr());
        if dp.is_null() {
            panic!("can't open directory {:?}", dir);
        }
        let mut dirp = readdir(dp);
        while !dirp.is_null() {
            // work around wrong osx binding
            let name = if cfg!(target_os = "macos") {
                let dirp2 = &mut *(dirp as *mut MyDirent);
                CStr::from_ptr(&(*dirp2).d_name[0]).to_str().unwrap()
            } else {
                CStr::from_ptr(&(*dirp).d_name[0]).to_str().unwrap()
            };
            println!("{}", name);
            dirp = readdir(dp);
        }
        closedir(dp);
    }
}
