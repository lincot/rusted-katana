//! <https://www.codewars.com/kata/596549c7743cf369b900021b/train/rust>

use core::hint::unreachable_unchecked;

/// checks if `x` is prime || `x` is divisible by 2 or 3 || `x` <= 1
/// given that `sqrt` is the square root of `x`
const fn is_prime_with_condition(x: u32, sqrt: u32) -> bool {
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

pub const fn twin_prime(n: i32) -> u32 {
    if n <= 3 {
        return 0;
    }
    let n = n as u32;

    let mut p = 5;
    let mut res = 1;

    let mut sqrt = 2;
    let mut next_perfect_square = 9;

    while p < n {
        if n >= next_perfect_square {
            sqrt += 1;
            next_perfect_square += 2 * sqrt + 1;
        }

        if is_prime_with_condition(p, sqrt) && is_prime_with_condition(p + 2, sqrt) {
            res += 1;
        }
        p += 6;
    }

    res
}
