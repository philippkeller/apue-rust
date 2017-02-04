/// Exercise 10.5: Using only a single timer (either alarm or the higher-precision setitimer),
/// provide a set of functions that allows a process to set any number of timers.
///
/// Basic idea: have a mutable static list (yeah.. but there's NO WAY to pass arguments to
/// signal handlers) of function pointers and "due alarm time"
///
/// On new alarm:
/// - add alarm to list
/// - sort list by "due alarm time"
///
/// When alarm goes off:
/// - deduct last alarm time from all alarms
/// - remove first element
///
/// What works: Start all alarms at the start ot the program
/// What doesn't: Start a few alarms, then wait and start a few others

extern crate libc;
use libc::{c_int, SIGALRM};
use libc::{signal, alarm};
use std::time::Duration;
use std::thread;

#[derive(Clone, Copy)]
struct Timeout {
    function: fn(),
    timeout_s: u32,
}

static mut SIGNALS: [Timeout; 16] = [Timeout {
    function: dummy,
    timeout_s: 999999,
}; 16];
static mut LAST_TIMEOUT_S: u32 = 0;

// empty function for filling signals
fn dummy() {}

fn sig_alrm(_: c_int) {
    unsafe {
        for (i, signal) in SIGNALS.iter().enumerate() {
            // cannot just do signal -= last_timeout_s as signal is not mutable
            // (couldn't find out how to make the static initialization with mutable objects,
            // sounds super hacky anyway so I gave up..)
            SIGNALS[i] = Timeout {
                function: signal.function,
                timeout_s: signal.timeout_s - LAST_TIMEOUT_S,
            };
        }
        let f = SIGNALS[0].function;
        // "remove" first element and put a dummy element at the end.
        // There are probably nicer ways to do that..
        SIGNALS[0] = Timeout {
            function: dummy,
            timeout_s: 999999,
        };
        SIGNALS.sort_by(|a, b| a.timeout_s.cmp(&b.timeout_s));
        LAST_TIMEOUT_S = SIGNALS[0].timeout_s;
        alarm(LAST_TIMEOUT_S);
        f();
    }
}

// timeout 1
// timeout 2
// ->
fn set_timeout(callback: fn(), timeout_s: u32) {
    unsafe {
        SIGNALS[15] = Timeout {
            function: callback,
            timeout_s: timeout_s,
        };
        SIGNALS.sort_by(|a, b| a.timeout_s.cmp(&b.timeout_s));
        signal(SIGALRM, sig_alrm as usize);
        LAST_TIMEOUT_S = SIGNALS[0].timeout_s;
        alarm(LAST_TIMEOUT_S);
    }
}

fn testme1() {
    println!("testme1 called");
}

fn testme2() {
    println!("testme2 called");
}

fn main() {
    set_timeout(testme2, 2);
    set_timeout(testme1, 1);
    thread::sleep(Duration::from_millis(4000));
}
