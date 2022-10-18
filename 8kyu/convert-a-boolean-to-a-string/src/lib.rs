//! <https://www.codewars.com/kata/551b4501ac0447318f0009cd/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;

#[allow(clippy::missing_const_for_fn)]
pub fn boolean_to_string(b: bool) -> String {
    if b {
        "true".into()
    } else {
        "false".into()
    }
}
