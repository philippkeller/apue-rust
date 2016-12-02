/// Figure 1.3 List all the files in a directory
///
/// Takeaways: Look more closely on the definition in the
/// headerfile. I missed the `$INODE64` bit first.
/// See [issue on rust-lang](https://github.com/rust-lang/libc/issues/414)
/// for details
///
/// $ mkdir -p /tmp/apue
/// $ touch /tmp/apue/hans
/// $ f03-list-files /tmp/apue | grep hans
/// hans
/// $ rm /tmp/apue/hans
/// $ rm -d /tmp/apue

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use apue::{array_to_string, LibcResult};
use apue::my_libc::readdir;
use libc::{opendir, closedir};

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
