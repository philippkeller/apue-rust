/// Exercise 11.1: Modify the example code shown in Figure 11.4 to pass the structure between the
/// threads properly.
///
/// The cleanest solution is - I think - if the main thread is responsible
/// for both mallocing and freeing the memory, thus we don't need
/// the return value (or pthread_exit), but store the changes in to the arg
/// variable directly.
///
/// The solution in the book works as well of course, but leaks memory.
///
/// $ e01-pass-struct
/// Foo { a: 55, b: 66, c: 3, d: 4 }

extern crate libc;
extern crate apue;
use libc::c_void;
use libc::usleep;
use apue::my_libc::pthread_create;
use apue::LibcResult;
use std::ptr::null_mut;

#[derive(Debug)]
struct Foo {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

unsafe extern "C" fn thr_fn1(foo_ptr: *mut c_void) -> *mut c_void {
    let foo = foo_ptr as *mut Foo;
    (*foo).a = 55;
    (*foo).b = 66;
    0 as _
}

fn main() {
    unsafe {
        let foo = Box::new(Foo {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
        });
        let mut tid1 = std::mem::uninitialized();
        let foo_ptr = Box::into_raw(foo);
        pthread_create(&mut tid1, null_mut(), thr_fn1, foo_ptr as *mut c_void).check_zero().expect("can't create thread 1");
        libc::pthread_join(tid1, null_mut()).check_zero().expect("join error");
        let foo: Box<Foo> = Box::from_raw(foo_ptr);
        usleep(100);
        println!("{:?}", foo);
    }
}
