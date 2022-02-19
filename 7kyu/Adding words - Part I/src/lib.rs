//! <https://www.codewars.com/kata/592eaf848c91f248ca000012/train/rust>

use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Arith {
    pub value: &'static str,
}

impl<'a, 'b> Add<&'a str> for &'b Arith {
    type Output = &'static str;

    fn add(self, rhs: &'a str) -> Self::Output {
        Arith::tostr(Arith::tonum(self.value) + Arith::tonum(rhs))
    }
}

impl Arith {
    fn tonum(s: &str) -> u8 {
        let mut s = s.bytes();

        match s.next().unwrap() {
            b'z' => 0,
            b'o' => 1,
            b't' => match s.next().unwrap() {
                b'w' => 2,
                b'h' => 3,
                _ => 10,
            },
            b'f' => match s.next().unwrap() {
                b'o' => 4,
                _ => 5,
            },
            b's' => match s.next().unwrap() {
                b'i' => 6,
                _ => 7,
            },
            b'e' => 8,
            _ => 9,
        }
    }

    const fn tostr(n: u8) -> &'static str {
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
}
