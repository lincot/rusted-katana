//! <https://www.codewars.com/kata/5681e4ff81ba1b0cdb000031/train/rust>

use core::hint::unreachable_unchecked;

use digital::prelude::*;

type Digits = heapless::Vec<u8, { u32::MAX_LEN_BASE10 }>;

pub fn find_int(a: u32, b: u32, k: u32) -> (usize, Vec<u32>) {
    if b < a {
        return (0, Vec::new());
    }

    let mut digits = Digits::new();
    unsafe {
        digits.write_int_unchecked_with::<ReversedRaw, Base10>(a);
    }

    let mut score_prod = (1u32..)
        .zip(digits.iter().rev())
        .map(|(c, &x)| c * x as u32)
        .sum();
    let mut score_pow = (1u32..)
        .zip(digits.iter().rev())
        .map(|(c, &x)| c.pow(x as u32))
        .sum();

    let mut res = Vec::new();

    if qualifies(score_prod, score_pow, k) {
        res.push(a);
    }

    for n in a + 1..=b {
        unsafe { increment(&mut digits, &mut score_prod, &mut score_pow) };
        if qualifies(score_prod, score_pow, k) {
            res.push(n);
        }
    }

    (res.len(), res)
}

const fn qualifies(score_prod: u32, score_pow: u32, k: u32) -> bool {
    (score_pow * k).is_multiple_of(sum_divisors(score_prod))
}

unsafe fn increment(digits: &mut Digits, score_prod: &mut u32, score_pow: &mut u32) {
    for (coef, digit) in (1..digits.len() as u32 + 1).rev().zip(digits.iter_mut()) {
        if *digit == 9 {
            *digit = 0;
            *score_prod -= 9 * coef;
            *score_pow -= coef.pow(9) - 1;
            continue;
        }

        *score_prod += coef;
        *score_pow += (coef - 1) * coef.pow(*digit as _);
        *digit += 1;
        return;
    }

    unsafe { digits.push_unchecked(1) };
    *score_prod = 1;
    *score_pow = digits.len() as u32;
}

const fn sum_divisors(mut n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    let mut res = 1;

    let pow_of_2 = n.trailing_zeros();
    res *= (1 << (pow_of_2 + 1)) - 1;
    n >>= pow_of_2;

    let mut x = 3;
    while x * x <= n {
        let mut term = 1;
        if x == 0 {
            unsafe { unreachable_unchecked() };
        }
        while n.is_multiple_of(x) {
            n /= x;
            term = term * x + 1;
        }
        res *= term;
        x += 2;
    }

    if n > 1 {
        res *= 1 + n;
    }

    res
}
