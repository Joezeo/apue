#![cfg(unix)]
mod chapter_1;
mod chapter_2;
mod chapter_3;
mod apue;

fn main() {
    chapter_1::main();
}

#[cfg(not(unix))]
fn main() {}