#![cfg(unix)]
mod chatper_1;
mod apue;

fn main() {
    chatper_1::main();
}

#[cfg(not(unix))]
fn main() {}