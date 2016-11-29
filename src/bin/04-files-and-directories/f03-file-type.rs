/// Figure 4.3: Print type of file for each command-line argument
///
/// Takeaways:
///
/// - S_ISREG et al are C macros. S_ISREG is defined as `(((m) & S_IFMT) == S_IFREG)`
///   since S_IFMT is 1111000000000000 we can just and this mask to the mode field
///   and match the result directly (avoiding else ifs)
/// - while let is awesome
///
/// $ ln -s /var/tmp /tmp/aaa
/// $ f03-file-type /etc/passwd /tmp/ /dev/null /tmp/aaa
/// mode of "/etc/passwd": regular
/// mode of "/tmp/": directory
/// mode of "/dev/null": character special
/// mode of "/tmp/aaa": symbolic link
/// $ rm /tmp/aaa

extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;

use libc::{stat, lstat, mode_t,S_IFMT,S_IFBLK,S_IFCHR,S_IFDIR,S_IFIFO,S_IFREG,S_IFLNK,S_IFSOCK};
use apue::{LibcResult};
use std::ffi::CString;

fn main() {
    let mut args = std::env::args();
    if args.len() == 1 {
        println!("usage:\n{} filenames\n\ne.g. /etc/passwd /tmp",
                 args.next().unwrap());
        std::process::exit(1);
    }
    args.next(); // skip filename
    while let Some(filename) = args.next() {
        unsafe {
            let mut buf: stat = std::mem::uninitialized();
            let s = CString::new(filename).unwrap();
            if let None = lstat(s.as_ptr(), &mut buf).to_option() {
                panic!("lstat error: {}", errno::errno());
            }
            let a:mode_t = buf.st_mode & S_IFMT;
            let t = match a {
                S_IFREG => "regular",
                S_IFBLK => "block special",
                S_IFCHR => "character special",
                S_IFDIR => "directory",
                S_IFIFO => "fifo",
                S_IFLNK => "symbolic link",
                S_IFSOCK => "socket",
                _ => "** unnown mode **"
            };
            println!("mode of {:?}: {}", s, t);
        }
    }
}