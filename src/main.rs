#![cfg(unix)]
mod chapter_1;
mod chapter_2;
mod apue;

fn main() {
    chapter_1::main();
}

#[cfg(not(unix))]
fn main() {}