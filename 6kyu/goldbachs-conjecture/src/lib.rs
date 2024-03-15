//! <https://www.codewars.com/kata/537ba77315ddd92659000fec/train/rust>

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

pub fn check_goldbach(n: u32) -> Option<(u32, u32)> {
    if n % 2 != 0 || n <= 2 {
        return None;
    }
    if n == 4 {
        return Some((2, 2));
    }

    let ([prime0, prime1], mut value0, mut step) = match n % 3 {
        0 => ([5, 7], 11, 2),
        1 => ([3, 5], 11, 6),
        2 => ([3, 7], 13, 6),
        _ => unreachable!(),
    };

    let n_sqrt = n.sqrt();
    let sqrt0 = if n_sqrt.pow(2) > n - prime0 {
        n_sqrt - 1
    } else {
        n_sqrt
    };
    if is_prime_with_condition(n - prime0, sqrt0) {
        return Some((prime0, n - prime0));
    }
    let sqrt1 = if sqrt0 == n_sqrt - 1 || n_sqrt.pow(2) > n - prime1 {
        n_sqrt - 1
    } else {
        n_sqrt
    };
    if is_prime_with_condition(n - prime1, sqrt1) {
        return Some((prime1, n - prime1));
    }

    let mut sqrt0 = 3u32;
    let mut next_perfect_square1 = (sqrt0 + 1).pow(2);
    let mut value1 = n - value0;
    let mut sqrt1 = if n_sqrt.pow(2) > value1 {
        n_sqrt - 1
    } else {
        n_sqrt
    };
    let mut next_perfect_square2 = sqrt1.pow(2);

    loop {
        if value0 >= next_perfect_square1 {
            sqrt0 += 1;
            next_perfect_square1 += 2 * sqrt0 + 1;
        }

        if value1 < next_perfect_square2 {
            sqrt1 -= 1;
            next_perfect_square2 -= 2 * sqrt1 - 1;
        }

        if is_prime_with_condition(value0, sqrt0) && is_prime_with_condition(value1, sqrt1) {
            return Some((value0, value1));
        }

        value0 += step;
        value1 -= step;

        if step != 6 {
            step ^= 6;
        }
    }
}
