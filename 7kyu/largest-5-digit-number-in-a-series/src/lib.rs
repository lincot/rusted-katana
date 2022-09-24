//! <https://www.codewars.com/kata/51675d17e0c1bed195000001/train/rust>

#![no_std]

pub fn largest_five_digit_number(num: &str) -> u32 {
    num.as_bytes()
        .windows(5)
        .max()
        .map(|digits| digits.iter().fold(0, |acc, d| 10 * acc + (d - b'0') as u32))
        .unwrap()
}
