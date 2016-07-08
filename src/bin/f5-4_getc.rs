extern crate libc;
extern crate apue;
use apue::LibcIntResult;

fn main() {
	let r = 'r' as i8;
	let w = 'w' as i8;
	unsafe {
		let stdin = libc::fdopen(libc::STDIN_FILENO, &r as *const libc::c_char);
		let stdout = libc::fdopen(libc::STDOUT_FILENO, &w as *const libc::c_char);
		while let Some(c) = libc::fgetc(stdin).to_option() {
			if libc::fputc(c, stdout) == libc::EOF {
				panic!("output error");
			}
		}
	    if libc::ferror(stdin) != 0 {
	    	panic!("input error");
	    }
	}
}