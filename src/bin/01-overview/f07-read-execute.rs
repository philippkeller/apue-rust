/// Figure 1.7 Read commands from standard input and execute them
///
/// Takeaways:
///
/// - this was astonishingly very straightforward to port to rust
///   without any stumbling blocks
/// - IMO the if let around fork() makes the code more readable
///   than the C counterpart. also the parent/child distinction
///   in the original C program should have been an if/else block
///
/// $ echo tty | f07-read-execute 2>&1
/// not a tty
/// % %

extern crate libc;
#[macro_use(cstr, as_char)]
extern crate apue;

use libc::{STDIN_FILENO, c_char, printf, strlen, fgets, fdopen, fork, waitpid};
use apue::{array_to_string, LibcResult};
use apue::my_libc::execlp;

const MAXLINE: usize = 100;

fn main() {
    unsafe {
        let mut buf: [c_char; MAXLINE] = std::mem::uninitialized();
        let stdin = fdopen(STDIN_FILENO, &('r' as c_char));
        let mut status = 0;
        printf(cstr!("%% ")); // print prompt (printf requires %% to print %)
        while !fgets(as_char!(buf), MAXLINE as _, stdin).is_null() {
            let len = strlen(as_char!(buf));
            if buf[len - 1] == '\n' as _ {
                buf[len - 1] = 0;
            }
            let pid = fork().check_not_negative().expect("fork error");
            if pid == 0 {
                // child
                execlp(as_char!(buf), as_char!(buf), 0);
                panic!("could not execute {}", array_to_string(&buf));
            } else {
                // parent
                waitpid(pid, &mut status, 0).check_not_negative().expect("waitpid error");
                printf(cstr!("%% "));
            }
        }
    }
}