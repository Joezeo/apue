#![allow(dead_code)]

use libc::{O_ACCMODE, O_RDONLY, O_WRONLY, O_RDWR, F_GETFL, O_APPEND, O_NONBLOCK, O_SYNC};

pub fn print_file_flag(fd: i32) {
    unsafe {
        let val = libc::fcntl(fd, F_GETFL, 0);
        match val & O_ACCMODE {
            O_RDONLY => print!("read only"),
            O_WRONLY => print!("write only"),
            O_RDWR => print!("read write"),
            _ => print!("unkonwn access mode"),
        }

        if val & O_APPEND > 0 {
            print!(", append");
        }
        if val & O_NONBLOCK > 0 {
            print!(", nonblocking");
        }
        if val & O_SYNC > 0 {
            print!(", sync");
        }
        println!();
    }
}