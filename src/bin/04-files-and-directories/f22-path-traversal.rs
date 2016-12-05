/// Figure 4.22: Recursively descend a directory hierarchy, counting file types
/// adapted to chdir into directories as suggested in Exercise 4.11
///
/// takeaways:
///
/// - calling the libc functions was straightforward (with the exception of readdir which
///   is missing in rusts libc (only readdir_r is there)
/// - first forgot to call `closedir` and ran into the "too many files open" limit
/// - the C code in the book suffers is hard to read. Rusts OOP features help a lot in making
///   the code easier to read:
///   + Counter struct instead of static variables and passing a function
///     pointer
///   + Enum instead of constants
///
/// timing wise this rust implementation is about the same as the c program without chdir.
/// (before implementing chdir it was about 100% slower, most probably due to the fact that
/// this implementation does too much string copying)
///
/// $ rm -rf /tmp/f21
/// $ mkdir /tmp/f21
/// $ touch /tmp/f21/{a,b,c,d,e}
/// $ ln -s /tmp /tmp/f21/tmp
/// $ mkdir /tmp/f21/0
/// $ touch /tmp/f21/0/{f,g,e}
/// $ f22-path-traversal /tmp/f21
///                8 regular files    72.73%
///                2 directories      18.18%
///                1 symbolic links    9.09%
///                0 block special     0.00%
///                0 char special      0.00%
///                0 FIFOs             0.00%
///                0 sockets           0.00%
/// $ rm -rf /tmp/f21

extern crate clap;
extern crate libc;
#[macro_use(print_err, cstr)]
extern crate apue;
extern crate errno;

use libc::{stat, S_IFMT, S_IFDIR, S_IFREG, S_IFBLK, S_IFCHR, S_IFIFO, S_IFLNK, S_IFSOCK, opendir,
           closedir, lstat, chdir};
use apue::LibcResult;
use apue::my_libc::readdir;
use clap::App;
use std::ffi::{CString, CStr};

struct Counter {
    nreg: u64,
    ndir: u64,
    nblk: u64,
    nchr: u64,
    nfifo: u64,
    nslink: u64,
    nsock: u64,
}

impl Counter {
    fn count_file(&mut self, path: &str, statbuf: &stat) {
        match statbuf.st_mode & S_IFMT {
            S_IFREG => self.nreg += 1,
            S_IFBLK => self.nblk += 1,
            S_IFCHR => self.nchr += 1,
            S_IFIFO => self.nfifo += 1,
            S_IFLNK => self.nslink += 1,
            S_IFSOCK => self.nsock += 1,
            // directories should have type = FTW_D
            S_IFDIR => panic!("found S_IFDIR for {}", path),
            _ => panic!("unknown stat: {}", statbuf.st_mode),
        }
    }
    fn count_other(&mut self, path: &str, typ: FileType) {
        match typ {
            FileType::Directory => self.ndir += 1,
            FileType::DirectoryCannotRead => print_err!("cannot read dir: {}", path),
            FileType::FileCannotStat => print_err!("cannot stat file: {}", path),
        };
    }
}

impl std::fmt::Debug for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let total = (self.nsock + self.ndir + self.nslink + self.nfifo + self.nblk +
                     self.nchr + self.nreg) as f32 / 100.0;
        write!(f,
               r#"        {0:8} regular files    {7:5.2}%
        {1:8} directories      {8:5.2}%
        {2:8} symbolic links   {9:5.2}%
        {3:8} block special    {10:5.2}%
        {4:8} char special     {11:5.2}%
        {5:8} FIFOs            {12:5.2}%
        {6:8} sockets          {13:5.2}%"#,
               self.nreg,
               self.ndir,
               self.nslink,
               self.nblk,
               self.nchr,
               self.nfifo,
               self.nsock,
               self.nreg as f32 / total,
               self.ndir as f32 / total,
               self.nslink as f32 / total,
               self.nblk as f32 / total,
               self.nchr as f32 / total,
               self.nfifo as f32 / total,
               self.nsock as f32 / total)
    }
}

enum FileType {
    Directory,
    DirectoryCannotRead,
    FileCannotStat,
}

unsafe fn myftw(cnt: &mut Counter) {
    let dp = match opendir(cstr!(".")).to_option() {
        Some(dp) => dp,
        None => {
            print_err!("cannot open dir: {}", errno::errno());
            cnt.count_other(".", FileType::DirectoryCannotRead);
            return;
        }
    };
    while let Some(dirp) = readdir(dp).to_option() {
        let filename = CStr::from_ptr((&(*dirp).d_name).as_ptr()).to_str().expect("invalid string");
        if filename == "." || filename == ".." {
            continue;
        }
        let mut statbuf: stat = std::mem::uninitialized();
        if let None = lstat(CString::new(filename.to_owned()).unwrap().as_ptr(),
                            &mut statbuf)
            .to_option() {
            cnt.count_other(&filename, FileType::FileCannotStat);
            continue;
        }
        match statbuf.st_mode & S_IFMT {
            S_IFDIR => {
                cnt.count_other(&filename, FileType::Directory);
                if let Some(_) = chdir(cstr!(filename)).to_option() {
                    myftw(cnt);
                    chdir(cstr!(".."));
                }
            }
            _ => cnt.count_file(&filename, &statbuf),
        }
    }
    if let None = closedir(dp).to_option() {
        panic!("can’t close directory {}", errno::errno());
    }
}

fn main() {
    unsafe {
        // put ndir=1 because we first start with the initial dir
        let mut c = Counter {
            nreg: 0,
            ndir: 1,
            nblk: 0,
            nchr: 0,
            nfifo: 0,
            nslink: 0,
            nsock: 0,
        };
        let matches = App::new("tree-traversal")
            .args_from_usage("<path> beginning path for traversal")
            .get_matches();
        let path = matches.value_of("path").unwrap();
        chdir(cstr!(path));
        myftw(&mut c);
        println!("{:?}", c);
    }
}
