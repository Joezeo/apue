use std::ffi::CString;

use libc::{close, fcntl, F_GETFL, open, O_RDWR, O_APPEND, O_CREAT, O_TRUNC, write, c_void, lseek, SEEK_CUR};

use crate::apue::FILE_MODE;

pub fn my_dup2(fd: i32, fd2: i32) -> i32 {
    unsafe {
        if fd2 == fd {
            return fd
        }

        close(fd2);

        let path = format!("/dev/fd/{}", fd);
        let cpath = CString::new(path).unwrap();
        let mode = fcntl(fd, F_GETFL);
        let mut cs = vec![];

        loop {
            let nfd = open(cpath.as_ptr(), mode);
            if nfd == -1 {
                return -1
            }
            if nfd == fd2 {
                break
            } else {
                cs.push(nfd)
            }
        }

        for cfd in cs {
            close(cfd);
        }

        fd2
    } 
}

pub fn append_check() {
    unsafe {
        let path = CString::new("append.check").unwrap();
        let fd = open(path.as_ptr(), O_RDWR | O_APPEND | O_CREAT | O_TRUNC, FILE_MODE);
        let content = "..................................................................................................................".as_bytes();
        write(fd, content.as_ptr() as *const c_void, content.len());
        lseek(fd, 5, SEEK_CUR);
        let append = "uuuuuuuu".as_bytes();
        write(fd, append.as_ptr() as *const c_void, append.len());
        close(fd);
    }
}