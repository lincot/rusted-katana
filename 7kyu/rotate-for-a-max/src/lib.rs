//! <https://www.codewars.com/kata/56a4872cbb65f3a610000026/train/rust>

use std::cmp::Ordering;

pub fn max_rot(n: u64) -> u64 {
    fn to_digits(mut n: u64) -> ([u8; 20], usize) {
        let (mut digits, mut len) = ([0; 20], 0);
        while n != 0 {
            unsafe { *digits.get_unchecked_mut(len) = (n % 10) as u8 };
            n /= 10;
            len += 1;
        }
        unsafe { digits.get_unchecked_mut(..len) }.reverse();
        if len > digits.len() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        (digits, len)
    }

    fn from_digits(digits: &[u8]) -> u64 {
        digits.iter().fold(0, |acc, &d| 10 * acc + d as u64)
    }

    let (mut digits, digits_len) = to_digits(n);
    let mut max_digits = digits;

    for keep in 0..digits_len {
        unsafe { digits.get_unchecked_mut(keep..digits_len) }.rotate_left(1);

        match unsafe {
            digits
                .get_unchecked(keep)
                .cmp(max_digits.get_unchecked(keep))
        } {
            Ordering::Greater => {
                unsafe {
                    max_digits
                        .get_unchecked_mut(keep..digits_len)
                        .copy_from_slice(digits.get_unchecked(keep..digits_len));
                }
                continue;
            }
            Ordering::Less => {
                break;
            }
            Ordering::Equal => {}
        }

        for i in keep + 1..digits_len {
            match unsafe { digits.get_unchecked(i).cmp(max_digits.get_unchecked(i)) } {
                Ordering::Greater => {
                    unsafe {
                        max_digits
                            .get_unchecked_mut(i..digits_len)
                            .copy_from_slice(digits.get_unchecked(i..digits_len));
                    }
                    break;
                }
                Ordering::Less => {
                    break;
                }
                Ordering::Equal => {}
            }
        }
    }

    from_digits(unsafe { max_digits.get_unchecked(..digits_len) })
}
