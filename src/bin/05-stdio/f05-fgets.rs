/// Figure 5.5: Copy standard input to standard output using fgets and fputs
///
/// $ echo kurt | f05-fgets
/// kurt

extern crate libc;

static MAXLINE: libc::c_int = 10;

fn main() {
    unsafe {
        let stdin = libc::fdopen(libc::STDIN_FILENO, &('r' as libc::c_char));
        let stdout = libc::fdopen(libc::STDOUT_FILENO, &('w' as libc::c_char));
        let mut buf = Vec::with_capacity(MAXLINE as usize);
        let ptr = buf.as_mut_ptr() as *mut libc::c_char;
        while !libc::fgets(ptr, MAXLINE, stdin).is_null() {
            if libc::fputs(ptr, stdout) == libc::EOF {
                panic!("output error");
            }
        }
        if libc::ferror(stdin) != 0 {
            panic!("input error");
        }
    }
}
