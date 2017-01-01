/// Exercise 8.3 Rewrite the program in Figure 8.6 to
/// use waitid instead of wait. Instead of calling
/// pr_exit, determine the equivalent information from
/// the siginfo structure.
///
/// Takeaway:
/// - I could not catch the exit signals 6 and 8 (the second and third fork)
///   it *should* have worked, the struct definition seems to match the headers in /usr/include..
/// - the same program in C (e.g. http://bit.ly/2iSLMid) yields the same results. On Linux
///   it works though, but on MacOs it prints signal number = 0 also for the 2nd and 3rd line
/// - on Linux the field `si_status` is missing in libc, since the headers on Linux are a mess..


extern crate apue;

use libc::{exit, fork, abort, raise, siginfo_t, SIGFPE};
use apue::my_libc::{waitid, idtype_t, WEXITED, CLD_DUMPED, CLD_EXITED, CLD_STOPPED};
use apue::LibcResult;
use std::panic;

fn handle_panic(e: &panic::PanicInfo) {
    match e.payload().downcast_ref::<String>() {
        Some(as_string) if as_string == "attempt to divide by zero" => {
            unsafe { raise(SIGFPE) };
        }
        _ => {
            panic!("unknown error occurred");
        }
    }
}

#[cfg(target_os = "macos")]
fn pr_exit(i: siginfo_t) {
    let sigcode = i.si_code;
    let status = i.si_status;
    match sigcode {
        CLD_EXITED => println!("normal termination, exit status = {}", status),
        CLD_STOPPED => println!("child stoped, signal number = {}", status),
        _ => {
            println!("abnormal termination, signal number = {}", status);
            if sigcode == CLD_DUMPED {
                println!("(core file generated)");
            }
        }
    }
}

/// si_status is missing in Linux in libc, no wonder: header files is a mess there
#[cfg(target_os = "linux")]
fn pr_exit(i: siginfo_t) {
    let sigcode = i.si_code;
    match sigcode {
        CLD_EXITED => println!("normal termination"),
        CLD_STOPPED => println!("child stoped"),
        _ => {
            println!("abnormal termination");
            if sigcode == CLD_DUMPED {
                println!("(core file generated)");
            }
        }
    }
}

fn main() {
    panic::set_hook(Box::new(handle_panic));
    unsafe {
        let mut siginfo: siginfo_t = std::mem::uninitialized();
        let mut pid = fork().to_option().expect("fork error");
        if pid == 0 {
            // child
            exit(7);
        }
        waitid(idtype_t::P_PID, pid as _, &mut siginfo, WEXITED).to_option().expect("waitid error");

        pr_exit(siginfo);

        pid = fork().to_option().expect("fork error");
        if pid == 0 {
            // child
            abort(); // generate SIGABRT
        }
        waitid(idtype_t::P_PID, pid as _, &mut siginfo, WEXITED).to_option().expect("waitid error");
        pr_exit(siginfo);

        pid = fork().to_option().expect("fork error");
        if pid == 0 {
            // child
            let z = 0;
            pid = 1 / z; // divide by 0 generates SIGFPE
        }

        waitid(idtype_t::P_PID, pid as _, &mut siginfo, WEXITED).to_option().expect("waitid error");
        pr_exit(siginfo);
    }
}
