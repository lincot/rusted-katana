//! <https://www.codewars.com/kata/5ce6728c939bf80029988b57/train/rust>

#![no_std]

extern crate alloc;

pub fn solve(s: &str) -> bool {
    if s.len() > (b'z' - b'a' + 1) as usize {
        return false;
    }

    let mut bytes = s.as_bytes().to_vec();
    bytes.sort_unstable();

    for i in 1..bytes.len() {
        if bytes[i - 1] + 1 != bytes[i] {
            return false;
        }
    }

    true
}
