/// Figure 8.17 Echo all command-line arguments and all environment strings
///
/// no libc, only std as it is only a helper program

use std::env;

fn main() {
    for (i, arg) in env::args().enumerate() {
        println!("argv[{}] = {}", i, arg);
    }
    for (key, value) in env::vars() {
        println!("{}={}", key, value);
    }
}
