I'm reading through "[Advanced Programming in the UNIX Environment](https://www.amazon.com/dp/0321637739)". My dislike of C caused me to learn Rust and port some of the examples and exercises from C to Rust.

Beware: this is my first Rust project. This code is far from beautiful and might have mistakes (e.g. dangling pointers). PR are *very* welcome. All code is tested on OS X and most on it also on Linux.

## Progress

- [x] Chapter 1: Overview
- [ ] Chapter 4: Files and Directories
- [x] Chapter 5: Standard I/O Library
- [x] Chapter 6: System Data Files and Information

for Chapter 1-4 see [Andelf's github repo](https://github.com/andelf/rust-apue)

## Building

Don't use `./Cargo.toml` for building, but instead build via `./build.sh` (that's a workaround around [this issue](https://github.com/rust-lang/cargo/issues/3138)).

Alternatively build via `cargo build --manifest-path macos/Cargo.toml` for OSX or `cargo build --manifest-path linux/Cargo.toml` for Linux.