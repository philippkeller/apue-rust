I'm reading through "[Advanced Programming in the UNIX Environment](https://www.amazon.com/dp/0321637739)". My dislike of C caused me to learn Rust and port some of the examples and exercises from C to Rust.

Beware: this is my first Rust project. This code is far from beautiful and might have mistakes (e.g. dangling pointers). PR are *very* welcome. All code is tested on OS X and most on it also on Linux.

## Progress

- [x] Chapter 1: Overview
- [x] Chapter 2: UNIX Standardization and Implementations
- [x] Chapter 3: File I/O
- [x] Chapter 4: Files and Directories
- [x] Chapter 5: Standard I/O Library
- [x] Chapter 6: System Data Files and Information
- [x] Chapter 7: Process Environment
- [ ] Chapter 8: Process Control

## Using this code

TODO: 

- can you copy-paste this code for your project? Mostly, yes, but some examples are probably optimal because
they copy data (e.g. buffers) even when unneeded.
- explain some basic usage `cstr!`, `to_option()` and working with buffers allocated via vectors.

## Building

- Building should work out of the box for Macos and Linux on rust nightly.
- Some of the binaries only do something on Macos, others only for Linux (see #cfg switches in the main methods)
- If you regularly switch between building MacOs and Linux you can tell cargo to put those files in different directories
  using `export CARGO_TARGET_DIR=target/linux`
  
## Code not ported to Rust:

- Figure 7.9, 7.11, 7.13: setjmp, longjmp: of course Rust solves this with exception handling (i.e. with explicit 
  error handling or using `panic::catch_unwind`). These sections (as well as the one about malloc, etc.) are
  actually very good reasons why to not use C directly but instead turn to something safer like Rust.
  In addition to that: Rust doesn't offer a way to safe unwind the stack after a longjump: 
  https://users.rust-lang.org/t/force-cleanup-before-longjmp/3376
- Figure 7.14: That's exactly why you take Rust over C because Rust will complain at compile time that you cannot
  return a stack variable from a function.