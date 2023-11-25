//! <https://www.codewars.com/kata/650a86e8404241005fc744ca/train/rust>

#![no_std]

pub const fn same_length(txt: &str) -> bool {
    let (mut balance, mut prev_is_one, mut i) = (0isize, -1isize, 0);
    while i < txt.len() {
        if txt.as_bytes()[i] == b'1' {
            if prev_is_one == -1 && balance != 0 {
                return false;
            }
            prev_is_one = 1;
        } else {
            prev_is_one = -1;
        }
        balance += prev_is_one;
        i += 1;
    }
    balance == 0
}
