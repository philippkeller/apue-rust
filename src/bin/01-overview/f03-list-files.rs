/// Figure 1.3 List all the files in a directory
///
/// Takeaways: Look more closely on the definition in the
/// headerfile. I missed the `$INODE64` bit first.
/// See [issue on rust-lang](https://github.com/rust-lang/libc/issues/414)
/// for details

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use apue::{array_to_string, LibcResult};
use libc::{opendir, closedir, DIR, dirent};

extern "C" {
    #[cfg(target_os = "macos")]
    #[link_name = "readdir$INODE64"]
    pub fn readdir(arg1: *mut DIR) -> *mut dirent;

    #[cfg(not(target_os = "macos"))]
    pub fn readdir(arg1: *mut DIR) -> *mut dirent;

}

fn main() {
    let dir = std::env::args().nth(1).expect("please specify a path");
    unsafe {
        let dp = opendir(cstr!(dir.clone()));
        assert!(!dp.is_null(), format!("can't open directory {:?}", dir));
        while let Some(dirp) = readdir(dp).to_option() {
            println!("{}", array_to_string(&(*dirp).d_name));
        }
        closedir(dp);
    }
}