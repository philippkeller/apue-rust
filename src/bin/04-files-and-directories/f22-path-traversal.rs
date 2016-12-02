/// Figure 4.22: Recursively descend a directory hierarchy, counting file types
///
/// TODO: explain why this is better than the C program, object oriented and yada yada
/// TODO: doctest

extern crate clap;
extern crate libc;
extern crate apue;
extern crate errno;

use libc::{stat, S_IFMT, S_IFDIR, S_IFREG, S_IFBLK, S_IFCHR,
    S_IFIFO, S_IFLNK, S_IFSOCK, opendir, closedir, lstat};
use apue::{LibcResult, array_to_string};
use apue::my_libc::readdir;
use clap::App;
use std::ffi::CString;

struct Counter {
    nreg:u64,
    ndir:u64,
    nblk:u64,
    nchr:u64,
    nfifo:u64,
    nslink:u64,
    nsock:u64,
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
            FileType::Directory => self.ndir+= 1,
            FileType::DirectoryCannotRead => println!("cannot read dir: {}", path),
            FileType::FileCannotStat => println!("cannot stat file: {}", path),
        };
    }
}

impl std::fmt::Debug for Counter {
    // TODO: percentages
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, r#"
        {:8} regular files
        {:8} directories
        {:8} symbolic links
        {:8} block special
        {:8} char special
        {:8} FIFOs
        {:8} sockets"#,
               self.nreg, self.ndir, self.nslink, self.nblk, self.nchr, self.nfifo, self.nsock)
    }
}

enum FileType {
    Directory,
    DirectoryCannotRead,
    FileCannotStat,
}

unsafe fn myftw(path: &str, cnt: &mut Counter) {
    let p = CString::new(path).unwrap();
    let dp = match opendir(p.as_ptr()).to_option() {
        Some(dp) => dp,
        None => {
            println!("cannot open dir: {}", errno::errno());
            cnt.count_other(path, FileType::DirectoryCannotRead);
            return
        },
    };
    while let Some(dirp) = readdir(dp).to_option() {
        let filename = array_to_string(&(*dirp).d_name);
        if filename == "." || filename == ".." {
            continue;
        }
        let filename = format!("{}/{}", path, filename);
        let mut statbuf:stat = std::mem::uninitialized();
        if let None = lstat(CString::new(filename.to_owned()).unwrap().as_ptr(), &mut statbuf).to_option() {
            cnt.count_other(&filename, FileType::FileCannotStat);
            continue;
        }
        match statbuf.st_mode & S_IFMT {
            S_IFDIR => {
                cnt.count_other(&filename, FileType::Directory);
                myftw(&filename, cnt);
            },
            _ => cnt.count_file(&filename, &statbuf),
        }
    }
    if let None = closedir(dp).to_option() {
        panic!("can’t close directory {}", path);
    }
}

fn main() {
    unsafe {
        // put ndir=1 because we first start with the initial dir
        let mut c = Counter { nreg: 0, ndir: 1, nblk: 0, nchr: 0, nfifo: 0, nslink: 0, nsock: 0};
        let matches = App::new("tree-traversal")
            .args_from_usage("<path> beginning path for traversal").get_matches();
        let path = matches.value_of("path").unwrap();
        myftw(path, &mut c);
        println!("{:?}", c);
    }
}

