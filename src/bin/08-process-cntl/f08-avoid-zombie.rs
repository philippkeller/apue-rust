/// Figure 8.8: Avoid zombie processes by calling fork twice
///
/// $ f08-avoid-zombie
/// $ f08-avoid-zombie && sleep 0.1
/// second child, parent pid = 1


extern crate libc;
extern crate apue;

use libc::{fork, usleep, waitpid, getppid, exit};
use apue::{LibcResult, err_sys};

// separate function to avoid one indent because of the unsafe block
unsafe fn doit() {
    let pid = fork().to_option().expect("fork error");
    match pid {
        0 => {
            let pid = fork().to_option().expect("fork error");
            if pid > 0 {
                // parent from second fork == first child
                exit(0);
            }
            // we're the second child; our parent becomes init
            // as soon as our real parent calls exit() (3 lines above)
            // After 2 seconds sleep our parent will be dead and
            // init will reap our status.
            usleep(10);
            println!("second child, parent pid = {}", getppid());
            exit(0);
        }
        _ => {
            // wait for first child
            if waitpid(pid, std::ptr::null_mut(), 0) != pid {
                err_sys("waitpid error");
            }
        }
    }
}

fn main() {
    unsafe { doit() };
}
