//! <https://www.codewars.com/kata/5539fecef69c483c5a000015/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::hint::unreachable_unchecked;
use num_integer::Roots;
use unchecked::PushUnchecked;

/// checks if `x` is prime || `x` is divisible by 2 or 3 || `x` <= 1
/// given that `sqrt` is the square root of `x`
const fn is_prime_with_condition(x: u64, sqrt: u64) -> bool {
    let mut divisor = 5;
    let mut step = 2;
    while divisor <= sqrt {
        if divisor == 0 {
            unsafe { unreachable_unchecked() };
        }
        if x % divisor == 0 {
            return false;
        }

        divisor += step;
        step ^= 6;
    }
    true
}

const fn reverse_digits(mut x: u64) -> (u64, u8, u8) {
    let mut res = 0;
    let mut log10 = u8::MAX;
    let mut last_digit = 0;
    while x != 0 {
        res *= 10;
        last_digit = (x % 10) as _;
        res += last_digit as u64;
        x /= 10;
        log10 = log10.wrapping_add(1);
    }
    (res, log10, last_digit)
}

pub fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    let parse_num = |n: u64| {
        let r = n % 6;
        let (next_r, step) = if r <= 1 { (1, 4) } else { (5, 2) };
        let next = n + next_r - r;
        let sqrt = next.sqrt();
        (next, step, sqrt, (sqrt + 1).pow(2))
    };

    let mut res = Vec::with_capacity(((stop - start) / 2 + 1) as _);

    let (mut start, mut step, mut sqrt, mut next_perfect_square) = parse_num(start);

    while start <= stop {
        if start >= next_perfect_square {
            sqrt += 1;
            next_perfect_square += 2 * sqrt + 1;
        }

        let (rev, log10, last_digit) = reverse_digits(start);

        if last_digit % 2 == 0 {
            start = (last_digit + 1) as u64 * 10u64.pow(log10 as _) + 1;
            (start, step, sqrt, next_perfect_square) = parse_num(start);
        } else {
            if start != rev
                && is_prime_with_condition(start, sqrt)
                && is_prime_with_condition(rev, rev.sqrt())
            {
                unsafe { res.push_unchecked(start) };
            }

            start += step;
            step ^= 6;
        }
    }

    res
}
