extern crate gcc;

fn main() {
    gcc::compile_library("libthread-cleanup.a", &["src/bin/11-threads/thread-cleanup.c"]);
    gcc::compile_library("libthread-barrier.a", &["src/bin/11-threads/thread-barrier.c"]);
    println!("cargo:rerun-if-changed=src/bin/11-threads/thread-cleanup.c");
    println!("cargo:rerun-if-changed=src/bin/11-threads/thread-barrier.c");
}