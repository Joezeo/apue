#![allow(dead_code)]
mod file_copy;
mod hole_file;
mod nohole_file;
mod is_seekable;
mod practice;
mod print_file_flag;
mod shared_file_table;

use std::ffi::CString;
use libc::{O_RDWR, O_APPEND, O_SYNC};

use crate::apue::{FILE_MODE, errno};

pub fn main() {
    unsafe {
        // open:
        let path = CString::new("test.file").unwrap();
        let fd = libc::open(path.as_ptr(), libc::O_RDWR);
        if *errno() != 0 {
            let prefix = CString::new("file open failed:").unwrap();
            libc::perror(prefix.as_ptr())
        } else {
            println!("fd: {}", fd)
        }
        libc::close(fd);

        // openat:
        let dir = CString::new("/Users/joezeo/Workspace/project/git/apue").unwrap();
        let dir = libc::open(dir.as_ptr(), libc::O_RDONLY | libc::O_DIRECTORY);
        println!("Dir fd: {}", dir as i32);

        // let fd = libc::openat(dir, path.as_ptr(), libc::O_RDWR);
        let fd = libc::openat(
            libc::AT_FDCWD, /* open in current working directory, */
            path.as_ptr(),
            libc::O_RDWR,
        );
        if *errno() != 0 {
            let prefix = CString::new("file open failed:").unwrap();
            libc::perror(prefix.as_ptr())
        } else {
            println!("fd: {}", fd)
        }
        libc::close(fd);
        libc::close(dir);

        let path = CString::new("/").unwrap();
        println!("file name/path truncate: {}", libc::pathconf(path.as_ptr(), libc::_PC_NO_TRUNC));

        // create:
        let path = CString::new("test_open.file").unwrap();
        // let fd = libc::creat(path.as_ptr(), FILE_MODE);
        let fd = libc::open(path.as_ptr(), libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC, FILE_MODE as libc::c_uint);

        // lseek:
        let curpos = libc::lseek(fd, 0, libc::SEEK_CUR);
        println!("File current offset: {}", curpos);

        is_seekable::is_seekable();

        // hole file:
        hole_file::hole_file();
        nohole_file::nohole_file();

        // copy file:
        file_copy::copy("file.nohole");

        // shared file table:
        shared_file_table::shared_file_table(true, false);
        shared_file_table::shared_file_table(false, false);

        // print file flag:
        let path = CString::new("test.file").unwrap();
        let fd = libc::open(path.as_ptr(), O_RDWR | O_APPEND | O_SYNC);
        print_file_flag::print_file_flag(fd);

        // practice:
        shared_file_table::shared_file_table(true, true);
        practice::append_check();
    }
}
