#![allow(dead_code)]
use std::ffi::{CStr, CString};

use crate::apue::errno;

pub fn error_info() {
    unsafe {
        println!(
            "EACESS: {}",
            CStr::from_ptr(libc::strerror(libc::EACCES))
                .to_str()
                .unwrap()
        );

        *errno() = libc::ENOENT;
        let prefix = CString::new("ENOENT").unwrap();
        libc::perror(prefix.as_ptr())
    }
}
