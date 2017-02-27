#![allow(unused_imports)]
/// Exercise: BSD-based systems provide a function called funopen that
/// allows us to intercept read, write, seek, and close calls
/// on a stream. Use this function to implement fmemopen
/// for FreeBSD and Mac OS X.
///
/// Solution: This is only a basic solution which handles
/// opening in a+, reading, writing and seeking and takes care that you don't
/// read/write over the buffers end.
/// The solution on page 913 in the book goes much further but I didn't find
/// motivation to go that far :-)
///
/// Takeaways from this exercise:
///
/// - write is only called when I call seek (maybe also close), so I guess
///   instead of calling write right away it's queued somehow
/// - when MemStream.buffer is not mutable the code compiles and there is no
///   segfault when writing to the buffer but the buffer is just not altered.
///   This was somehow unexpected (and a segfault would have been helpful here),
///   but when changing buffer to a mut it all worked as expected.
/// - handing around the struct as a pointer is scary, but I guess still nicer
///   than a public singleton (which isn't really supported by rust).
///   As the struct instance outlives the pointer (is only dropped at end of main)
///   we're safe here
///
/// mac only:
///
/// $ e07-fmemopen-bsd 2>/dev/null
/// buffer = aaaaaaaaaaaaaaaaaaaaaaaaaaaa
/// mem buffer = lorem ipsum doloraaaaaaaaaaa
/// mem buffer = lorem ipsuhansaplast!aaaaaaa

extern crate libc;
#[macro_use(cstr)]
extern crate apue;

#[cfg(any(target_os = "macos", target_os= "bsd"))]
mod fmemopen {
    use libc::{c_void, c_char, c_int, off_t, FILE, SEEK_SET, memset, memcpy, fgets, fputs, printf,
               fseek};
    use std::option::Option;
    use std::cmp::min;
    use std::mem::uninitialized;

    pub const BUFLEN: usize = 30;

    pub struct MemStream<'a> {
        buffer: &'a mut [u8; BUFLEN],
        pos: u8,
    }

    extern "C" {
        fn funopen(cookie: *mut c_void,
                   readfn: Option<unsafe extern "C" fn(cookie: *mut c_void,
                                                       buffer: *mut c_char,
                                                       nbyte: c_int)
                                                       -> c_int>,
                   writefn: Option<unsafe extern "C" fn(cookie: *mut c_void,
                                                        buffer: *mut c_char,
                                                        nbyte: c_int)
                                                        -> c_int>,
                   seekfn: Option<unsafe extern "C" fn(cookie: *mut c_void,
                                                       offset: off_t,
                                                       whence: c_int)
                                                       -> off_t>,
                   closefn: Option<unsafe extern "C" fn(cookie: *mut c_void) -> c_int>)
                   -> *mut FILE;
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

    unsafe fn max_bytes(nbyte: u8, status: *mut MemStream) -> u8 {
        min(nbyte, ((*status).buffer.len() as u8 - (*status).pos))
    }

    unsafe extern "C" fn read(cookie: *mut c_void, buffer: *mut c_char, nbyte: c_int) -> c_int {
        let status = cookie as *mut MemStream;
        let nbyte = max_bytes(nbyte as u8, status);
        if nbyte > 0 {
            memcpy(buffer as _, (*status).buffer.as_void(), nbyte as usize);
            (*status).pos += nbyte;
            nbyte as _
        } else {
            -1
        }
    }

    unsafe extern "C" fn write(cookie: *mut c_void, buffer: *mut c_char, nbyte: c_int) -> c_int {
        let status = cookie as *mut MemStream;
        let nbyte = max_bytes(nbyte as u8, status);
        if nbyte > 0 {
            memcpy((*(*status).buffer).as_void().offset((*status).pos as _),
                   buffer as *mut _,
                   nbyte as usize);
            (*status).pos += nbyte;
            nbyte as _
        } else {
            -1
        }
    }

    unsafe extern "C" fn seek(cookie: *mut c_void, offset: off_t, _: c_int) -> off_t {
        let status = cookie as *mut MemStream;
        (*status).pos = offset as u8;
        offset
    }

    unsafe fn fmemopen(cookie: *mut c_void) -> *mut FILE {
        funopen(cookie, Some(read), Some(write), Some(seek), None)
    }

    pub fn runit() {
        unsafe {
            let mut buf: [u8; BUFLEN] = uninitialized();
            memset(buf.as_ptr() as *mut c_void, 'a' as c_int, BUFLEN - 2);
            buf[BUFLEN - 2] = '\0' as u8;
            buf[BUFLEN - 1] = 'X' as u8;
            printf(cstr!("buffer = %s\n"), buf.as_ptr());
            let mut status = MemStream {
                buffer: &mut buf,
                pos: 0,
            };
            // could be written as &mut status as *mut _ as *mut _
            // so the types would be coerced but for clarity I wrote
            // out the types. Two casts are neeeded because first
            // the reference is cast into a pointer and then the pointer
            // type is changed
            let cookie = &mut status as *mut MemStream as *mut c_void;
            let fd = fmemopen(cookie);

            // let status2 = cookie as *mut MemStream;
            // let mut buf:[u8;BUFLEN] = *((*status2).buffer);
            // println!("buf = {:?}", buf);
            // buf[0] = 55;
            // println!("buf = {:?}", buf);

            let tmpbuf: [u8; 5] = [55; 5];
            fputs("lorem ipsum dolor\0".as_ptr() as _, fd);
            fseek(fd, 10, SEEK_SET);
            printf(cstr!("mem buffer = %s\n"), status.buffer.as_ptr());
            fputs(cstr!("hansaplast!") as _, fd);
            fseek(fd, 0, SEEK_SET);
            printf(cstr!("mem buffer = %s\n"), status.buffer.as_ptr());
            while !fgets(tmpbuf.as_ptr() as *mut i8, 5, fd).is_null() {
                printf(cstr!("buffer = %s\n"), tmpbuf.as_ptr());
            }
        }
    }
}


fn main() {
    #[cfg(any(target_os = "macos", target_os= "bsd"))]
    fmemopen::runit();
}
