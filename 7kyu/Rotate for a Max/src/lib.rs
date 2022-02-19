//! <https://www.codewars.com/kata/56a4872cbb65f3a610000026/train/rust>

use std::cmp::Ordering;

pub fn max_rot(n: u64) -> u64 {
    fn to_digits(mut n: u64) -> Vec<u8> {
        let mut digits = Vec::with_capacity(20);

        while n != 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }

        digits.reverse();
        digits
    }

    fn from_digits(digits: Vec<u8>) -> u64 {
        digits.into_iter().fold(0, |acc, d| 10 * acc + d as u64)
    }

    let mut digits = to_digits(n);
    let mut max_digits = digits.clone();

    for keep in 0..digits.len() {
        digits[keep..].rotate_left(1);

        if digits.len() != max_digits.len() {
            unsafe { core::hint::unreachable_unchecked() };
        }

        match digits[keep].cmp(&max_digits[keep]) {
            Ordering::Greater => {
                max_digits[keep..].copy_from_slice(&digits[keep..]);
                continue;
            }
            Ordering::Less => {
                break;
            }
            _ => {}
        }

        for i in keep + 1..digits.len() {
            match digits[i].cmp(&max_digits[i]) {
                Ordering::Greater => {
                    max_digits[i..].copy_from_slice(&digits[i..]);
                    break;
                }
                Ordering::Less => {
                    break;
                }
                _ => {}
            }
        }
    }

    from_digits(max_digits)
}
