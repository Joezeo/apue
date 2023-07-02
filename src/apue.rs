#![allow(dead_code)]
use libc::{c_int, mode_t, S_IRGRP, S_IROTH, S_IRUSR, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};

pub const MAXLINE: c_int = 4096;

/// Default file access permissions for new files.
pub const FILE_MODE: mode_t = S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH;

/// Default permissions for new firectories.
pub const DIR_MODE: mode_t = FILE_MODE | S_IXUSR | S_IXGRP | S_IXOTH;
