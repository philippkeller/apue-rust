I'm reading through "[Advanced Programming in the UNIX Environment](https://www.amazon.com/dp/0321637739)". My dislike of C caused me to learn Rust and port some of the examples and exercises from C to Rust.

Beware: this is my first Rust project. This code is far from beautiful and might have mistakes (e.g. dangling pointers). PR are *very* welcome. All code is tested on OS X and most on it also on Linux.

## Progress

- [x] Chapter 1: Overview
- [x] Chapter 2: UNIX Standardization and Implementations
- [ ] Chapter 3: File I/O
- [ ] Chapter 4: Files and Directories
- [x] Chapter 5: Standard I/O Library
- [x] Chapter 6: System Data Files and Information

for Chapter 1-4 see [Andelf's github repo](https://github.com/andelf/rust-apue)

## Building

- Building should work out of the box for Macos and Linux on rust nightly.
- Some of the binaries only do something on Macos, others only for Linux (see #cfg switches in the main methods)
- If you regularly switch between building MacOs and Linux you can tell cargo to put those files in different directories
  using `export CARGO_TARGET_DIR=target/linux`