#![allow(dead_code)]
use std::{ffi::CString, time::Instant};
use libc::*;
use crate::apue::FILE_MODE;

const BUFFSIZE: usize = 256;

pub fn copy(file: &str) {
    unsafe {
        let instant = Instant::now();
        let path = CString::new(file).unwrap();
        let fd = open(path.as_ptr(), O_RDONLY);

        if fd == -1 {
            panic!("open file {} failed.", file)
        }

        let mut file = file.to_string();
        file.push_str(".copy");
        let write_path = CString::new(file).unwrap();
        let write_fd = open(write_path.as_ptr(), O_RDWR | O_CREAT | O_TRUNC, FILE_MODE as c_uint);

        let mut buf = vec!['\0'; BUFFSIZE];
        loop {
            let n = read(fd, buf.as_mut_ptr() as *mut c_void, BUFFSIZE);
            if n <= 0 {
                break;
            }
            let writed = write(write_fd, buf.as_mut_ptr() as *mut c_void, n as usize);
            if writed != n {
                panic!("write error.")
            }
        }
        println!("copy file time comsuption: {}μs", instant.elapsed().as_micros())
    }
}