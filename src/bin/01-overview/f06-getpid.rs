/// Figure 1.6 Print the process ID
///
/// $ f06-getpid > /dev/null

extern crate libc;

fn main() {
    println!("hello world from process ID {}", unsafe { libc::getpid() });
}
