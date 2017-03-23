/// Figure 2.14 Print all possible sysconf and pathconf values
///
/// How apue solves this is by writing a program in awk with 70+ printf commands
/// which generates a C program which itself has a #ifdef for every possible PC/SC constant.
/// First I found this very ugly and second #ifdef AFAIK doesn't work in Rust.
///
/// My approach: Parse the header files which contain the `PC` and `SC` definition
/// (binding to an int) and print those via sysconf or pathconf.
/// The header file differ for
/// OSX (/usr/include/sys/unistd.h for PC, /usr/include/unistd.h for SC) and
/// Linux (/usr/include/unistd.h only). The crux is that in Linux the constants are inside
/// an enum so this program tries to emulate that enum which of course is very error prone.
///
/// Current state: works for OSX, and Linux (PC_: seems 100% correct, SC_ seems correct until
/// there is a off by 1 error around _SC_THREAD_SAFE_FUNCTIONS)
///
/// to validate the results compare it against `getconf -a`
///
/// $ f14-limits | grep _PC_NAME_MAX
/// _PC_NAME_MAX =  255


extern crate libc;
#[macro_use(cstr)]
extern crate apue;
extern crate errno;
extern crate regex;

use libc::{EINVAL, puts, printf, pathconf, sysconf};
use apue::{LibcResult, uname};
use errno::errno;
use std::io::{BufReader, BufRead};
use regex::Regex;

fn pr_conf(key: &str, path: &str, name: i32) {
    unsafe {
        printf(cstr!("%s = "), cstr!(key));
        let res = match &key[0..4] {
            "_SC_" => sysconf(name),
            "_PC_" => pathconf(cstr!(path), name),
            _ => {
                panic!("key needs to start either with _PC_ or _SC_ but was {:?}",
                       key)
            }
        };
        if let Ok(val) = res.check_not_negative() {
            printf(cstr!(" %ld\n"), val);
        } else {
            let e = errno();
            let _ = match e.0 {
                EINVAL => {
                    puts(cstr!(" (not supported)"));
                }
                0 => {
                    puts(cstr!(" (no limit)"));
                }
                _ => panic!("pathconf error, path = {:?}", path),
            };
        }
    }
}

fn parse_header(hfile: &str) {
    let f = match std::fs::File::open(hfile) {
        Ok(file) => file,
        Err(_) => return,
    };
    let file = BufReader::new(&f);
    let re = Regex::new(r"^\s*#define\s+(_[PS]C_\w+)\s+(\w+).*").unwrap();
    let mut pc_counter = 0;
    let mut sc_counter = 0;
    let mut define_line = false;
    for line in file.lines() {
        if let Ok(line) = line {
            if let Some(groups) = re.captures(&line) {
                if groups.len() == 3 {
                    let key = &groups[1];
                    let val = &groups[2];
                    let val = match val.parse::<i32>() {
                        Ok(n) => n,
                        // assumption: if the definition is not a number, then
                        // we're inside an enum and just count up
                        // that's highly unstable of course..
                        Err(_) => {
                            if key.starts_with("_PC_") {
                                // if there are two #define after each other
                                // there's no increase in the enum value
                                // e.g. here:
                                // #define _SC_PAGESIZE                    _SC_PAGESIZE
                                // #define _SC_PAGE_SIZE                   _SC_PAGESIZE
                                if !define_line {
                                    pc_counter += 1;
                                }
                                pc_counter - 1 // enum starts with zero
                            } else {
                                if !define_line {
                                    sc_counter += 1;
                                }
                                sc_counter - 1
                            }
                        }
                    };
                    pr_conf(key, "/", val);
                    define_line = true;
                }
            } else {
                define_line = false;
            }
        }
    }
}

fn main() {
    match uname().unwrap().as_str() {
        "Linux" => {
            println!("linux!");
            parse_header("/usr/include/x86_64-linux-gnu/bits/confname.h");
        }
        "Darwin" => {
            parse_header("/usr/include/sys/unistd.h"); // for _PC_*
            parse_header("/usr/include/unistd.h"); // for _SC_*
        }
        _ => panic!("{:?} is not supported", uname()),
    }
}
