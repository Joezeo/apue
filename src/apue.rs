#![allow(dead_code)]
use std::sync::atomic::AtomicI32;

use libc::{
    c_char, c_int, mode_t, size_t, sysconf, S_IRGRP, S_IROTH, S_IRUSR, S_IWUSR, S_IXGRP, S_IXOTH,
    S_IXUSR,
};

pub const MAXLINE: c_int = 4096;

/// Default file access permissions for new files.
pub const FILE_MODE: mode_t = S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH;

/// Default permissions for new firectories.
pub const DIR_MODE: mode_t = FILE_MODE | S_IXUSR | S_IXGRP | S_IXOTH;

lazy_static::lazy_static!(
    pub static ref POSIX_VERSION: i64 = unsafe { sysconf(libc::_SC_VERSION) };
    pub static ref XSI_VERSION: i64 = unsafe { sysconf(libc::_SC_XOPEN_VERSION) };
);
pub static mut PATHMAX: AtomicI32 = AtomicI32::new(libc::PATH_MAX);

pub fn path_alloc(sizep: size_t) -> *const c_char {
    todo!()
}
