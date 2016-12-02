#![allow(dead_code)]
extern crate clap;
extern crate libc;
extern crate apue;
extern crate errno;

use libc::{stat, S_IFMT, S_IFDIR, opendir, lstat};
use apue::{LibcResult, array_to_string};
use apue::my_libc::readdir;
use clap::App;
use std::ffi::CString;

struct Counter {
    nreg:u16,
    ndir:u16,
    nblk:u16,
    nchr:u16,
    nfifo:u16,
    nslink:u16,
    nsock:u16,
    ntot:u16,
}

const FTW_F:u8 =   1; // file other than directory
const FTW_D:u8 =   2; // directory
const FTW_DNR:u8 = 3;
const FTW_NS:u8 =  4;

fn myfunc() {
    println!("called");
}

unsafe fn myftw(path: &str, func: &Fn()) {
    let p = CString::new(path).unwrap();
    let dp = opendir(p.as_ptr()).to_option().expect(&format!("could not open {:?}", path));
    while let Some(dirp) = readdir(dp).to_option() {
        let filename = array_to_string(&(*dirp).d_name);
        if filename == "." || filename == ".." {
            continue;
        }
        let filename = CString::new(format!("{}/{}", path, filename)).unwrap();
        let mut statbuf:stat = std::mem::uninitialized();
        if let None = lstat(filename.as_ptr(), &mut statbuf).to_option() {
            println!("lstat error: {}", errno::errno());
            // TODO: count lstat error
            continue;
        }
        match statbuf.st_mode & S_IFMT {
            S_IFDIR => println!("dir: {:?}", filename),
            _ => println!("file: {:?}", filename),
        }
    }
}

fn main() {
    unsafe {
        let c = Counter { nreg: 0, ndir: 0, nblk: 0, nchr: 0, nfifo: 0, nslink: 0, nsock: 0, ntot: 0 };
        let matches = App::new("tree-traversal")
            .args_from_usage("<path> beginning path for traversal").get_matches();
        let path = matches.value_of("path").unwrap();
        let ret = myftw(path, &myfunc);
    }
}

