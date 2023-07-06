#![allow(dead_code)]
use std::ffi::CString;
use lazy_static::lazy_static;
use libc::*;

use crate::apue::FILE_MODE;

lazy_static! {
    pub static ref BUF1: CString = CString::new("abcdefghij").unwrap();
    pub static ref BUF2: CString = CString::new("ABCDEFGHIJ").unwrap();
}

pub fn hole_file() {
    unsafe {
        let path = CString::new("file.hole").unwrap();
        let fd = creat(path.as_ptr(), FILE_MODE);
        if fd < 0 {
            panic!("create error.")
        }

        if write(fd, BUF1.as_ptr() as *const c_void, 10) != 10 {
            panic!("buf1 write error.")
        }

        if lseek(fd, 16384, SEEK_SET) == -1 {
            panic!("lseek error.")
        }
 
        if write(fd, BUF2.as_ptr() as *const c_void, 10) != 10 {
            panic!("buf2 write error.")
        }
    }
}
