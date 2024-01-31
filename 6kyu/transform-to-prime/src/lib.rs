//! <https://www.codewars.com/kata/5a946d9fba1bb5135100007c/train/rust>

use core::hint::unreachable_unchecked;
use num_integer::Roots;

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

pub fn minimum_number(xs: &[u32]) -> u32 {
    let sum = xs.iter().sum();

    match sum {
        0 => return 2,
        1 => return 1,
        2 | 3 => return 0,
        _ => {}
    }

    let mut m = sum;

    let r = m % 6;
    let (next_r, mut step) = if r <= 1 { (1, 4) } else { (5, 2) };
    m += next_r - r;

    let mut sqrt = m.sqrt();
    let mut next_perfect_square = (sqrt + 1).pow(2);

    loop {
        if m >= next_perfect_square {
            sqrt += 1;
            next_perfect_square += 2 * sqrt + 1;
        }

        if is_prime_with_condition(m, sqrt) {
            return m - sum;
        }

        m += step;
        step ^= 6;
    }
}
