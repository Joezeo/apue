use std::ffi::{CString, CStr};

pub fn ls(dir: &'static str) {
    unsafe {
        let c_dir = CString::new(dir).unwrap();
        let dp = libc::opendir(c_dir.as_ptr());
        if dp.is_null() {
            println!("Cannot open {}", dir);
        }

        let mut dirp = libc::readdir(dp);
        while !dirp.is_null() {
            let cname = CStr::from_ptr((*dirp).d_name.as_ptr());
            println!("{}", cname.to_str().unwrap());
            dirp = libc::readdir(dp);
        }
    }
}
