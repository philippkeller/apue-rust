/// Figure 1.6 Print the process ID

extern crate libc;

fn main() {
    println!("hello world from process ID {}", unsafe { libc::getpid() });
}
