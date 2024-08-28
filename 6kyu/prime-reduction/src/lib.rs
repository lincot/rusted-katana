//! <https://www.codewars.com/kata/59aa6567485a4d03ff0000ca/train/rust>

use core::hint::unreachable_unchecked;
use num_integer::Roots;

pub fn solve(mut a: u32, b: u32) -> usize {
    if a < 7 {
        a = 7;
    }
    let r = a % 6;
    let (next_r, mut step) = if r <= 1 { (1, 4) } else { (5, 2) };
    a += next_r - r;
    let mut sqrt = a.sqrt();
    let mut next_perfect_square = (sqrt + 1).pow(2);
    let mut res = 0;
    while a < b {
        if a >= next_perfect_square {
            sqrt += 1;
            next_perfect_square += 2 * sqrt + 1;
        }

        if reduces_to_one(a) && is_prime_with_condition(a as _, sqrt as _) {
            res += 1;
        }

        a += step;
        step ^= 6;
    }
    res
}

#[inline]
const fn sum_squared_digits(mut x: u32) -> u32 {
    const SQUARED_DIGIT_SUMS_100: [u8; 100] = [
        0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 1, 2, 5, 10, 17, 26, 37, 50, 65, 82, 4, 5, 8, 13, 20,
        29, 40, 53, 68, 85, 9, 10, 13, 18, 25, 34, 45, 58, 73, 90, 16, 17, 20, 25, 32, 41, 52, 65,
        80, 97, 25, 26, 29, 34, 41, 50, 61, 74, 89, 106, 36, 37, 40, 45, 52, 61, 72, 85, 100, 117,
        49, 50, 53, 58, 65, 74, 85, 98, 113, 130, 64, 65, 68, 73, 80, 89, 100, 113, 128, 145, 81,
        82, 85, 90, 97, 106, 117, 130, 145, 162,
    ];

    let mut res = 0;
    while x >= 10 {
        res += SQUARED_DIGIT_SUMS_100[(x % 100) as usize] as u32;
        x /= 100;
    }
    res += SQUARED_DIGIT_SUMS_100[x as usize] as u32;
    res
}

const fn reduces_to_one(mut x: u32) -> bool {
    x = sum_squared_digits(x);
    while x > 6 {
        if x % 10 == 5 {
            return false;
        }
        x = sum_squared_digits(x);
    }
    x == 1
}

/// checks if `x` is prime || `x` is divisible by 2 or 3 || `x` <= 1
/// given that `sqrt` is the square root of `x`
const fn is_prime_with_condition(x: u16, sqrt: u16) -> bool {
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
