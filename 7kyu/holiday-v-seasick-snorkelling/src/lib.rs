//! <https://www.codewars.com/kata/57e90bcc97a0592126000064/train/rust>

#![no_std]

pub fn sea_sick(sea: &str) -> &'static str {
    let mut changes = 0;

    let sea = sea.as_bytes();

    for i in 1..sea.len() {
        if unsafe { sea.get_unchecked(i - 1) != sea.get_unchecked(i) } {
            changes += 1;
        }
    }

    if changes > sea.len() / 5 {
        "Throw Up"
    } else {
        "No Problem"
    }
}
