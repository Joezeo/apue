#![allow(dead_code)]
use libc::*;
pub fn is_seekable() {
    unsafe {
        if lseek(STDIN_FILENO, 0, SEEK_CUR) == -1 {
            println!("Cannot seek.")
        } else {
            println!("Seek OK.")
        }
    }
}