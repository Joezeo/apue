#![allow(dead_code)]
use std::{
    ffi::{c_int, CString},
    io::{stdin, BufRead},
};
use libc::{sighandler_t, SIG_ERR};

pub fn simple_shell() {
    unsafe {
        let mut status = 0;
        let stdin = stdin();
        let mut stdin_guard = stdin.lock();
        let mut line = String::new();

        // Add handel to signal
        if libc::signal(libc::SIGINT, singal_int as sighandler_t) == SIG_ERR {
            println!("SIG_ERR received.")
        }

        while let Ok(size) = stdin_guard.read_line(&mut line) {
            if size == 0 {
                break;
            }
            let pid = libc::fork();
            if pid < 0 {
                println!("fork error.")
            }

            if pid == 0 {
                let args: Vec<CString> = line
                    .trim()
                    .split_whitespace()
                    .map(|arg| CString::new(arg).unwrap())
                    .collect();
                let args_ptr: Vec<*const libc::c_char> =
                    args.iter().map(|arg| arg.as_ptr()).collect();

                if args.len() > 0 {
                    libc::execvp(args_ptr[0], args_ptr.as_ptr());
                    libc::exit(127);
                }
            }

            if libc::waitpid(pid, &mut status, 0) < 0 {
                println!("wait pid error")
            }
            line.clear();
            println!("%% ");
        }
    }
}

extern "C" fn singal_int(singal: c_int) {
    println!("Handle signal: {}", singal)
}
