//! <https://www.codewars.com/kata/5a9078e24a6b340b340000b8/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::sqrtf64;

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

pub fn solve(n: u64) -> u64 {
    match n {
        0..=2 => return 2,
        3..=4 => return 3,
        _ => {}
    }

    let r = n % 6;
    let (next_r, mut step) = if r <= 1 { (1, 4) } else { (5, 2) };
    let left_is_closer = [0, 2, 3].contains(&r);
    let mut right = n + next_r - r;
    let mut left = right - (step ^ 6);

    loop {
        if left_is_closer && is_prime_with_condition(left) {
            return left;
        }

        if is_prime_with_condition(right) {
            if r == 2 && step == 2 && is_prime_with_condition(left - step) {
                return left - step;
            }
            return right;
        }

        if !left_is_closer && is_prime_with_condition(left) {
            if r == 5 && step == 2 && is_prime_with_condition(right + step) {
                return right + step;
            }
            return left;
        }

        left -= step;
        right += step;
        step ^= 6;
    }
}
