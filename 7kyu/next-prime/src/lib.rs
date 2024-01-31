//! <https://www.codewars.com/kata/58e230e5e24dde0996000070/train/rust>

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

pub fn next_prime(mut n: u64) -> u64 {
    match n {
        0 | 1 => return 2,
        2 => return 3,
        _ => {}
    }

    let (first_step, mut step) = match n % 6 {
        0 => (1, 4),
        1 => (4, 2),
        2 => (3, 2),
        3 => (2, 2),
        4 => (1, 2),
        5 => (2, 4),
        _ => unreachable!(),
    };
    n += first_step;

    let mut sqrt = n.sqrt();
    let mut next_perfect_square = (sqrt + 1).pow(2);

    loop {
        if n >= next_perfect_square {
            sqrt += 1;
            next_perfect_square += 2 * sqrt + 1;
        }

        if is_prime_with_condition(n, sqrt) {
            return n;
        }

        n += step;
        step ^= 6;
    }
}
