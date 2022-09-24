//! <https://www.codewars.com/kata/592eaf848c91f248ca000012/train/rust>

#![no_std]

use core::ops::Add;

const fn ston(s: &str) -> u8 {
    let s = s.as_bytes();

    match s[0] {
        b'z' => 0,
        b'o' => 1,
        b't' => match s[1] {
            b'w' => 2,
            b'h' => 3,
            _ => 10,
        },
        b'f' => match s[1] {
            b'o' => 4,
            _ => 5,
        },
        b's' => match s[1] {
            b'i' => 6,
            _ => 7,
        },
        b'e' => 8,
        _ => 9,
    }
}

const fn ntos(n: u8) -> &'static str {
    match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fiveteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "twenty",
    }
}

#[derive(Clone, Copy)]
pub struct Arith {
    pub value: &'static str,
}

impl Add<&str> for &Arith {
    type Output = &'static str;

    fn add(self, rhs: &str) -> Self::Output {
        ntos(ston(self.value) + ston(rhs))
    }
}
