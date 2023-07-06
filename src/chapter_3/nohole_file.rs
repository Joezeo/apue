#![allow(dead_code)]
use std::ffi::CString;
use lazy_static::lazy_static;
use libc::*;
use crate::apue::FILE_MODE;

const LEN_SIZE_T: size_t = 16394;
const LEN_SSIZE_T: ssize_t = 16394;

lazy_static!{
    pub static ref BUF: CString = {
        let mut chars = vec![];
        for _ in 0..LEN_SIZE_T {
            let mut c: u8 = rand::random();
            c %= 26;
            c += b'a';
            chars.push(c as char);
        }
        let string: String = chars.into_iter().collect();
        CString::new(string.as_str()).unwrap()
    };
}

pub fn nohole_file() {
    unsafe {
        let path = CString::new("file.nohole").unwrap();
        let fd = creat(path.as_ptr(), FILE_MODE);
        if write(fd, BUF.as_ptr() as *const c_void, LEN_SIZE_T) != LEN_SSIZE_T {
            panic!("write buf error.")
        }
    }
}