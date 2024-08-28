//! <https://www.codewars.com/kata/59e0dbb72a7acc3610000017/train/rust>

use unchecked_std::prelude::*;

pub fn coprimes(n: u32) -> Vec<u32> {
    let prime_factors = get_prime_factors(n);

    let mut res = Vec::with_capacity(n as usize);
    for x in 1..n {
        if !prime_factors
            .iter()
            .any(|&p| unsafe { x.checked_rem(p).unwrap_unchecked() } == 0)
        {
            unsafe { res.push_unchecked(x) };
        }
    }
    res
}

fn get_prime_factors(mut n: u32) -> heapless::Vec<u32, 8> {
    let mut res = heapless::Vec::<_, 8>::new();

    let pow_of_2 = n.trailing_zeros();
    if pow_of_2 != 0 {
        unsafe { res.push_unchecked(2) };
    }
    n >>= pow_of_2;

    let mut x = 3;
    let mut n_sqrt = unsafe { (n as f64).sqrt().to_int_unchecked() };
    while x <= n_sqrt {
        let mut n_changed = false;

        while unsafe { n.checked_rem(x).unwrap_unchecked() } == 0 {
            n /= x;
            n_changed = true;
        }

        if n_changed {
            unsafe { res.push_unchecked(x) };
            n_sqrt = unsafe { (n as f64).sqrt().to_int_unchecked() };
        }
        x += 2;
    }

    let len = res.len();
    if len != 0 && n > 1 {
        unsafe { res.push_unchecked(n) };
    }

    res
}
