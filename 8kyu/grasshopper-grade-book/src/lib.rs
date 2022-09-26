//! <https://www.codewars.com/kata/55cbd4ba903825f7970000f5/train/rust>

#![no_std]

pub const fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    match (s1 + s2 + s3) / 3 {
        0..=59 => 'F',
        60..=69 => 'D',
        70..=79 => 'C',
        80..=89 => 'B',
        _ => 'A',
    }
}
