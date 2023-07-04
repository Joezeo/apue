#![allow(dead_code)]
use crate::apue::{path_alloc, open_max};

pub fn main() {
    let mut size = 0;
    let _ptr = path_alloc(&mut size);

    let _openmax = open_max();
}