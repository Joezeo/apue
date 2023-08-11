#![allow(dead_code)]
use libc::{c_uint, c_void, O_CREAT, O_TRUNC, O_WRONLY, F_DUPFD};
use std::ffi::CString;

use crate::{apue::FILE_MODE, chapter_3::practice::my_dup2};

pub fn shared_file_table(shared: bool, use_my_dup: bool) {
    unsafe {
        const BUFFER_SIZE: usize = 4096;

        let file = if shared {
            "file.shared"
        } else {
            "file.nonshared"
        };
        let path = CString::new(file).unwrap();
        let fd1 = libc::open(
            path.as_ptr(),
            O_WRONLY | O_CREAT | O_TRUNC,
            FILE_MODE as c_uint,
        );
        if fd1 == -1 {
            panic!("open file `{}` failed.", file)
        }
        let fd2 = if shared {
            if use_my_dup {
                // libc::dup(fd1)
                libc::fcntl(fd1, F_DUPFD, 0)
            } else {
                my_dup2(fd1, 3)
            }
        } else {
            libc::open(path.as_ptr(), O_WRONLY)
        };

        let content = "Hello, World!\n";
        let mut buf = vec![0u8; BUFFER_SIZE];
        let mut cnt = 0;
        for bt in content.bytes() {
            buf[cnt] = bt;
            cnt += 1;
        }

        // if `shared`, fd1 and fd2 point to the same file table.
        libc::write(fd1, buf.as_ptr() as *const c_void, cnt);
        libc::write(fd2, buf.as_ptr() as *const c_void, cnt);

        libc::close(fd2);
        libc::close(fd1);
    }
}
