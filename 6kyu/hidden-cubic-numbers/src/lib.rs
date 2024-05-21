//! <https://www.codewars.com/kata/55031bba8cba40ada90011c4/train/rust>

use core::hint::unreachable_unchecked;
use digital::{MaxLenBase10, WriteNumUnchecked};
use either::Either;
use unchecked_std::prelude::*;

pub fn is_sum_of_cubes(s: &str) -> String {
    let s = s.as_bytes();

    let mut numbers = Vec::new();
    let mut sum = 0usize;

    let mut i = 0;
    while i < s.len().saturating_sub(2) {
        if i + 2 > s.len() {
            unsafe { unreachable_unchecked() };
        }

        if s[i].is_ascii_digit() {
            if s[i + 1].is_ascii_digit() {
                if unsafe { s.get_unchecked(i + 2) }.is_ascii_digit() {
                    if &s[i..i + 3] == b"153" {
                        sum += 153;
                        numbers.push(Either::Left(*b"153"));
                    } else if &s[i..i + 3] == b"370" {
                        sum += 370;
                        numbers.push(Either::Left(*b"370"));
                    } else if &s[i..i + 3] == b"371" {
                        sum += 371;
                        numbers.push(Either::Left(*b"371"));
                    } else if &s[i..i + 3] == b"407" {
                        sum += 407;
                        numbers.push(Either::Left(*b"407"));
                    } else if &s[i..i + 3] == b"001" {
                        sum += 1;
                        numbers.push(Either::Right(b'1'));
                    } else if &s[i..i + 3] == b"000" {
                        numbers.push(Either::Right(b'0'));
                    }
                    i += 3;
                    continue;
                }
                if &s[i..i + 2] == b"01" {
                    numbers.push(Either::Right(b'1'));
                    sum += 1;
                } else if &s[i..i + 2] == b"00" {
                    numbers.push(Either::Right(b'0'));
                }
                i += 3;
                continue;
            }
            if s[i] == b'1' {
                numbers.push(Either::Right(b'1'));
                sum += 1;
            } else if s[i] == b'0' {
                numbers.push(Either::Right(b'0'));
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    if i + 1 < s.len() {
        if unsafe { s.get_unchecked(i..i + 2) } == b"01" {
            numbers.push(Either::Right(b'1'));
        } else if unsafe { s.get_unchecked(i..i + 2) } == b"00" {
            numbers.push(Either::Right(b'0'));
        } else if s[i + 1] == b'1' {
            sum += 1;
            numbers.push(Either::Right(b'1'));
        } else if s[i + 1] == b'0' {
            numbers.push(Either::Right(b'0'));
        }
    } else if i < s.len() {
        if s[i] == b'1' {
            sum += 1;
            numbers.push(Either::Right(b'1'));
        } else if s[i] == b'0' {
            numbers.push(Either::Right(b'0'));
        }
    }

    if numbers.is_empty() {
        return "Unlucky".into();
    }

    let mut res = Vec::with_capacity(4 * numbers.len() + usize::MAX_LEN_BASE10 + " Lucky".len());
    unsafe {
        for number in numbers {
            number.either_with(
                &mut res,
                |res, l| res.extend_from_slice_unchecked(&l),
                |res, r| res.push_unchecked(r),
            );
            res.push_unchecked(b' ');
        }

        res.write_num_unchecked(sum, 10, false, false);

        res.extend_from_slice_unchecked(b" Lucky");

        String::from_utf8_unchecked(res)
    }
}
