//! <https://www.codewars.com/kata/592e830e043b99888600002d/train/rust>

pub fn encode(msg: String, mut n: i32) -> Vec<i32> {
    let mut digits = Vec::with_capacity(9);

    while n != 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }

    msg.bytes()
        .map(|b| b - b'a' + 1)
        .zip(digits.into_iter().rev().cycle())
        .map(|(b, d)| (b + d) as _)
        .collect()
}
