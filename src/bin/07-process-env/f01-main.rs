/// Figure 7.1: Classic C program
///
/// When compiling main.c with gcc the return code is always 0 (most probably because
/// -std=c99 is implicitely set), same for the Rust code below
///
/// $ f01-main ; echo $?
/// hello world
/// 0


fn main() {
    println!("hello world");
}