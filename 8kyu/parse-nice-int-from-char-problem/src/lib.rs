//! <https://www.codewars.com/kata/557cd6882bfa3c8a9f0000c1/train/rust>

pub fn get_age(age: &str) -> u32 {
    (age.bytes().next().unwrap() - b'0') as _
}
