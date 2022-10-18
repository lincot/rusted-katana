//! <https://www.codewars.com/kata/5772da22b89313a4d50012f7/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

#[allow(clippy::missing_const_for_fn)]
pub fn greet(name: &str, owner: &str) -> String {
    if name == owner {
        "Hello boss".into()
    } else {
        "Hello guest".into()
    }
}
