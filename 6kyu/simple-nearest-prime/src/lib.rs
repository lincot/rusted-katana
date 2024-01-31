//! <https://www.codewars.com/kata/5a9078e24a6b340b340000b8/train/rust>

use core::hint::unreachable_unchecked;
use num_integer::Roots;

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

    let mut left_sqrt = left.sqrt();
    let mut left_prev_perfect_square = left_sqrt.pow(2);

    let mut right_sqrt = left_sqrt + ((left_sqrt + 1).pow(2) <= right) as u64;
    let mut right_next_perfect_square = (right_sqrt + 1).pow(2);

    loop {
        if left < left_prev_perfect_square {
            left_sqrt -= 1;
            left_prev_perfect_square -= 2 * left_sqrt + 1;
        }
        if right >= right_next_perfect_square {
            right_sqrt += 1;
            right_next_perfect_square += 2 * right_sqrt + 1;
        }

        if left_is_closer && is_prime_with_condition(left, left_sqrt) {
            return left;
        }

        if is_prime_with_condition(right, right_sqrt) {
            if r == 2
                && step == 2
                && is_prime_with_condition(
                    left - step,
                    left_sqrt - (left_sqrt.pow(2) > left - step) as u64,
                )
            {
                return left - step;
            }
            return right;
        }

        if !left_is_closer && is_prime_with_condition(left, left_sqrt) {
            if r == 5
                && step == 2
                && is_prime_with_condition(
                    right + step,
                    right_sqrt + ((right_sqrt + 1).pow(2) <= right + step) as u64,
                )
            {
                return right + step;
            }
            return left;
        }

        left -= step;
        right += step;
        step ^= 6;
    }
}
