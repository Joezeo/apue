#![allow(dead_code)]
pub fn user_info() {
    unsafe {
        println!("uid = {}, gid = {}", libc::getuid(), libc::getgid())
    }
}