//! <https://www.codewars.com/kata/56a4872cbb65f3a610000026/train/rust>

#![no_std]

use core::hint::unreachable_unchecked;
use digital::WriteNumUnchecked;

pub fn max_rot(n: u64) -> u64 {
    fn to_digits(n: u64) -> heapless::Vec<u8, 20> {
        let mut digits = heapless::Vec::new();
        unsafe { digits.write_num_unchecked(n, 10, true, true) };
        digits
    }

    fn from_digits(digits: &[u8]) -> u64 {
        digits.iter().rev().fold(0, |acc, &d| 10 * acc + d as u64)
    }

    let mut digits = to_digits(n);
    let mut max_digits = digits.clone();

    for end in (1..digits.len()).rev() {
        if end >= digits.len() {
            unsafe { unreachable_unchecked() };
        }

        digits[..=end].rotate_right(1);

        if digits[end] != max_digits[end] {
            if digits[end] > max_digits[end] {
                max_digits[..=end].copy_from_slice(&digits[..=end]);
                continue;
            }
            break;
        }

        for i in (0..end).rev() {
            unsafe {
                if digits.get_unchecked(i) != max_digits.get_unchecked(i) {
                    if digits.get_unchecked(i) > max_digits.get_unchecked(i) {
                        max_digits
                            .get_unchecked_mut(..=i)
                            .copy_from_slice(digits.get_unchecked_mut(..=i));
                    }
                    break;
                }
            }
        }
    }

    from_digits(&max_digits)
}
