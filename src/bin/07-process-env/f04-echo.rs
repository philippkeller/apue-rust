#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![no_main]

/// Figure 7.4: Echo all command-line arguments to standard output
///
/// Of course you would never do it this "raw" way (but rather with env::args())
/// but it was a nice exercise of dereferencing C pointers
///
/// $ f04-echo arg1 TEST foo
/// argv[0]: f04-echo
/// argv[1]: arg1
/// argv[2]: TEST
/// argv[3]: foo

extern crate libc;

use libc::printf;

#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) {
    unsafe {
        for i in 0.._argc {
            printf("argv[%d]: %s\n\0".as_ptr() as *const i8,
                   i,
                   *_argv.offset(i as _) as *const i8);
        }
    }
}