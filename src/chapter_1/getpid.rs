#![allow(dead_code)]
pub fn getpid() {
    unsafe {
        println!("pid: {}", libc::getpid());
    }
}
