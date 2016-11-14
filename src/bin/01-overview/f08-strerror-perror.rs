/// Figure 1.8 Demonstrate strerror and perror
///
/// Takeaways:
///
/// - I was somehow surprised that I cannot access errno with only libc
///   [this discussion](https://github.com/rust-lang/rfcs/pull/1571) explains
///   why it is in the separate errno crate.
/// - Without the extra fflush, on OSX the perror() output is printed first, only then the fprintf.
///   On linux it's the "right" way around.

extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;
use errno::{Errno, set_errno};

use libc::{EACCES, ENOENT, STDERR_FILENO, c_char, fdopen, fprintf, strerror, perror, fflush};

fn main() {
    unsafe {
        let stderr = fdopen(STDERR_FILENO, &('w' as c_char));
        fprintf(stderr, cstr!("EACCES: %s\n"), strerror(EACCES));
        fflush(stderr);
        set_errno(Errno(ENOENT));
        perror(cstr!(std::env::current_exe().unwrap().to_str().unwrap()));
    }
}
