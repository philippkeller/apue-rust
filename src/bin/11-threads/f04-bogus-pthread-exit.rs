/// Figure 11.4 Incorrect use of pthread_exit argument
///
/// Findings:
/// - behaves the same way as described in the book: on OSX it's
///   segfaulting and on Linux the memory is already overwritten and thus
///   printing bogus numbers
/// - The pointer handling in main was quite tricky to do in Rust,
///   I opened http://stackoverflow.com/questions/42235980 but
///   in the end didn't end up with Box::into_raw/from_raw as it behaved
///   differently than the solution below
/// - When working with a static mut FOO_GLOBAL; when saving Foo to this
///   static mut in thr_fn1, then it works of course, since this variable
///   is not dropped at the end of thr_fn1

extern crate libc;
extern crate apue;
use libc::c_void;
use libc::{pthread_self, pthread_join, usleep};
use apue::my_libc::{pthread_exit, pthread_create};
use apue::LibcResult;
use std::ptr::null_mut;

#[derive(Debug)]
struct Foo {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

extern "C" fn thr_fn1(_: *mut c_void) -> *mut c_void {
    let foo = Foo {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
    };
    printfoo("thread 1:", &foo);
    unsafe { pthread_exit(&foo as *const Foo as _) };
    0 as _
}

extern "C" fn thr_fn2(_: *mut c_void) -> *mut c_void {
    unsafe {
        println!("thread 2: ID is {}", pthread_self());
        pthread_exit(0 as _);
    }
    0 as _
}


fn printfoo(s: &str, fp: *const Foo) {
    unsafe {
        let raw = fp as *const i32;
        println!("{} structure at {:?}: {:?}", s, raw, &*fp);
    }

}

fn main() {
    unsafe {
        let mut fp: Box<Foo> = std::mem::uninitialized();
        let (mut tid1, mut tid2) = std::mem::zeroed();
        pthread_create(&mut tid1, null_mut(), thr_fn1, null_mut()).check_zero().expect("can't create thread 1");
        pthread_join(tid1, &mut fp as *mut _ as *mut _).check_zero().expect("can't join thread 1");
        usleep(100);
        println!("parent starting second thread");
        pthread_create(&mut tid2, null_mut(), thr_fn2, null_mut()).check_zero().expect("can't create thread 2");
        usleep(100);
        printfoo("parent:", fp.as_ref());

    }
}
