#![allow(dead_code)]
pub mod getpid;
pub mod ls;
pub mod simple_shell;
pub mod error;
pub mod user;

pub fn main() {
    // getpid::getpid();
    // ls::ls("/");
    // simple_shell::simple_shell();
    // error::error_info();
    user::user_info();
}