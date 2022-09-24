//! <https://www.codewars.com/kata/5a1a76c8a7ad6aa26a0007a0/train/rust>

#![no_std]

pub fn life_path_number(s: &str) -> u32 {
    const fn sum_digits(n: u8) -> u8 {
        if n > 9 {
            n / 10 + n % 10
        } else {
            n
        }
    }

    let s = s.as_bytes();

    let mut year = s[0..4].iter().sum::<u8>() - 4 * b'0';
    year = sum_digits(year);

    let month = s[5..7].iter().sum::<u8>() - 2 * b'0';

    let mut day = s[8..10].iter().sum::<u8>() - 2 * b'0';
    day = sum_digits(day);

    let mut sum = year + month + day;
    sum = sum_digits(sum);
    sum = sum_digits(sum);

    sum as _
}
