//! <https://www.codewars.com/kata/5539fecef69c483c5a000015/train/rust>

#![no_std]
#![feature(core_intrinsics)]

extern crate alloc;
use alloc::vec::Vec;
use core::intrinsics::sqrtf64;
use prelude::*;

/// checks if `x` is prime || `x` is divisible by 2 or 3 || `x` <= 1
fn is_prime_with_condition(x: u64) -> bool {
    let mut divisor = 5;
    let mut step = 2;
    while divisor <= unsafe { sqrtf64(x as _).to_int_unchecked() } {
        if divisor == 0 {
            unsafe { core::hint::unreachable_unchecked() };
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
    const fn get_next_and_step(start: u64) -> (u64, u64) {
        let r = start % 6;
        let (next_r, step) = if r <= 1 { (1, 4) } else { (5, 2) };
        (start + next_r - r, step)
    }

    let mut res = Vec::with_capacity(((stop - start) / 2 + 1) as _);

    let (mut start, mut step) = get_next_and_step(start);

    while start <= stop {
        let (rev, log10, last_digit) = reverse_digits(start);

        if last_digit % 2 == 0 {
            start = (last_digit + 1) as u64 * 10u64.pow(log10 as _) + 1;
            (start, step) = get_next_and_step(start);
        } else {
            if start != rev && is_prime_with_condition(start) && is_prime_with_condition(rev) {
                unsafe { res.push_unchecked(start) };
            }

            start += step;
            step ^= 6;
        }
    }

    res
}
