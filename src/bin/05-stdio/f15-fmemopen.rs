#![allow(unused_imports, dead_code)]

/// Figure 5.15: Investigate memory stream write behavior
///
/// This works for Linux only (MacOS/BSD don't have fmemopen implemented)
/// Translation C->Rust was very straight forward, only caveat
/// is the glibc bug that causes fflush to reset the file pointer position
///
/// linux only:
/// $ f15-fmemopen
/// initial buffer contents:
/// before flush:
/// after fflush:
/// len of string in buf = 0
/// after fseek: bbbbbbbbbbbbhello, world
/// len of string in buf = 24
/// after fclose: hello, worldcccccccccccccccccccccccccccccccccc
/// len of string in buf = 46

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

use libc::{c_void, size_t, c_char, c_uchar, c_int, FILE, SEEK_SET, memset, fprintf, fseek, fflush,
           strlen, printf, fclose};

const BSZ: usize = 48;
extern "C" {
    pub fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE;
}

trait CArray {
    fn as_muti8(&self) -> *mut i8;
    fn as_void(&self) -> *mut c_void;
}

impl CArray for [u8] {
    fn as_muti8(&self) -> *mut i8 {
        self.as_ptr() as *mut i8
    }
    fn as_void(&self) -> *mut c_void {
        self.as_ptr() as *mut c_void
    }
}

#[cfg(target_os = "linux")]
fn main() {
    unsafe {
        let mut buf: [c_uchar; BSZ] = std::mem::uninitialized();
        memset(buf.as_void(), 'a' as c_int, BSZ - 2);
        buf[BSZ - 2] = '\0' as u8;
        buf[BSZ - 1] = 'X' as u8;
        let fp = fmemopen(buf.as_ptr() as *mut c_void, BSZ, cstr!("w+"));
        if fp.is_null() {
            panic!("fmemopen failed");
        }
        printf(cstr!("initial buffer contents: %s\n"), buf.as_muti8());
        printf(cstr!("before flush: %s\n"), buf.as_muti8());
        fflush(fp);
        // fflush resets the position of the fp, that's a bug:
        // https://sourceware.org/bugzilla/show_bug.cgi?id=20005
        fseek(fp, ("hello world".len() + 1) as i64, SEEK_SET);
        printf(cstr!("after fflush: %s\n"), buf.as_muti8());
        printf(cstr!("len of string in buf = %ld\n"),
               strlen(buf.as_muti8()));

        memset(buf.as_void(), 'b' as c_int, BSZ - 2);
        buf[BSZ - 2] = '\0' as u8;
        buf[BSZ - 1] = 'X' as u8;
        fprintf(fp, cstr!("hello, world"));
        fseek(fp, 0, SEEK_SET);

        printf(cstr!("after fseek: %s\n"), buf.as_muti8());
        printf(cstr!("len of string in buf = %ld\n"),
               strlen(buf.as_muti8()));

        memset(buf.as_void(), 'c' as c_int, BSZ - 2);
        buf[BSZ - 2] = '\0' as u8;
        buf[BSZ - 1] = 'X' as u8;
        fprintf(fp, cstr!("hello, world"));
        fclose(fp);
        printf(cstr!("after fclose: %s\n"), buf.as_muti8());
        printf(cstr!("len of string in buf = %ld\n"),
               strlen(buf.as_muti8()));
    }
}

#[cfg(target_os = "macos")]
fn main() {
    unimplemented!();
}
