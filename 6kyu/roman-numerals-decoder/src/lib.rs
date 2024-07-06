//! <https://www.codewars.com/kata/51b6249c4612257ac0000005/train/rust>

pub fn roman_as_num(roman: &str) -> u64 {
    let roman = roman.as_bytes();
    let mut res = 0;
    let mut prev_digit = 0;
    for i in (0..roman.len()).rev() {
        let digit = match roman[i] {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            _ => 1000,
        };
        if digit >= prev_digit {
            res += digit;
        } else {
            res -= digit;
        }
        prev_digit = digit;
    }
    res
}
