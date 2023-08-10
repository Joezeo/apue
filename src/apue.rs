#![allow(dead_code)]
use libc::{
    c_char, c_int, mode_t, size_t, sysconf, S_IRGRP, S_IROTH, S_IRUSR, S_IWUSR, S_IXGRP, S_IXOTH,
    S_IXUSR, F_GETFL, fcntl, F_SETFL,
};
use std::{
    ffi::CString,
    sync::atomic::{AtomicI32, Ordering, AtomicI64},
};

pub const MAXLINE: c_int = 4096;

/// Default file access permissions for new files.
pub const FILE_MODE: mode_t = S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH;

/// Default permissions for new firectories.
pub const DIR_MODE: mode_t = FILE_MODE | S_IXUSR | S_IXGRP | S_IXOTH;

#[inline]
pub fn errno() -> *mut i32 {
    #[cfg(target_os = "macos")]
    unsafe {
        libc::__error()
    }
    #[cfg(not(target_os = "macos"))]
    unsafe {
        libc::__errno_location()
    }
}

pub fn path_alloc(sizep: &mut size_t) -> *mut c_char {
    const PATH_MAX_GUESS: i32 = 1024;

    lazy_static::lazy_static!(
        static ref POSIX_VERSION: i64 = unsafe { sysconf(libc::_SC_VERSION) };
        static ref XSI_VERSION: i64 = unsafe { sysconf(libc::_SC_XOPEN_VERSION) };
    );
    static PATHMAX: AtomicI32 = AtomicI32::new(libc::PATH_MAX);

    unsafe {
        if PATHMAX.load(Ordering::Relaxed) == 0 {
            *errno() = 0;
            let path = CString::new("/").unwrap();
            let pathmax = libc::pathconf(path.as_ptr(), libc::_PC_PATH_MAX);
            if pathmax < 0 {
                if *errno() == 0 {
                    PATHMAX.store(PATH_MAX_GUESS, Ordering::Release);
                } else {
                    panic!("pathconf error for _PC_PATH_MAX")
                }
            } else {
                PATHMAX.store(pathmax as i32, Ordering::Release);
            }
        }

        let pathmax = PATHMAX.load(Ordering::Relaxed) as size_t;
        let size = if *POSIX_VERSION < 200112 && *XSI_VERSION < 4 {
            pathmax + 1
        } else {
            pathmax
        };

        let ptr = libc::malloc(size);
        if ptr.is_null() {
            panic!("malloc error for pathname")
        }

        *sizep = size;

        ptr as *mut c_char
    }
}

pub fn open_max() -> i64 {
    static OPENMAX: AtomicI64 = AtomicI64::new(0);
    const OPEN_MAX_GUESS: i64 = 256;
    unsafe {
        if OPENMAX.load(Ordering::Acquire) == 0 {
            *errno() = 0;
            let openmax = sysconf(libc::_SC_OPEN_MAX);
            if openmax < 0 {
                if *errno() == 0 {
                    OPENMAX.store(OPEN_MAX_GUESS, Ordering::Release);
                } else {
                    panic!("sysconf error for _SC_OPEN_MAX")
                }
            } else {
                OPENMAX.store(openmax, Ordering::Release);
            }
        }

        return OPENMAX.load(Ordering::Acquire)
    }
}

pub fn set_fl(fd: i32, flags: i32) {
    unsafe {
        let mut val = fcntl(fd, F_GETFL);
        if val < 0 {
            println!("fcntl `F_GETFL` error.");
            return
        }
        val |= flags;

        if fcntl(fd, F_SETFL, val) < 0 {
            println!("fcntl `F_SETFL` error.");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::apue::open_max;

    use super::path_alloc;

    #[test]
    fn path_alloc_test() {
        let mut size = 0;
        let _ptr = path_alloc(&mut size);
    }

    #[test]
    fn open_max_test() {
        println!("{}", open_max())
    }
}
