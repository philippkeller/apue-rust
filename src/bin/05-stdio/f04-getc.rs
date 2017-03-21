/// Figure 5.4: Copy standard input to standard output using getc and putc
///
/// $ echo hans | f04-getc
/// hans

extern crate libc;
extern crate apue;
use apue::LibcResult;


fn main() {
    unsafe {
        // the first parameter could also be written as CString::new("r").unwrap().as_ptr()
        // but as it's only one character the cast below is shorter
        let stdin = libc::fdopen(libc::STDIN_FILENO, &('r' as libc::c_char));
        let stdout = libc::fdopen(libc::STDOUT_FILENO, &('w' as libc::c_char));
        while let Ok(c) = libc::fgetc(stdin).check_not_negative() {
            if libc::fputc(c, stdout) == libc::EOF {
                panic!("output error");
            }
        }
        if libc::ferror(stdin) != 0 {
            panic!("input error");
        }
    }
}
