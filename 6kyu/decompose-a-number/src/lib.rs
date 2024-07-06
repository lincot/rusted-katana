//! <https://www.codewars.com/kata/55ec80d40d5de30631000025/train/rust>

use unchecked_std::prelude::*;

pub fn decompose(mut n: u32) -> (Vec<u32>, u32) {
    if n < 4 {
        return (Vec::new(), n);
    }

    // largest coef count is 63 at n = 3_263_723_125
    let mut res = Vec::with_capacity(63);

    let log2 = u32::BITS - 1 - n.leading_zeros();
    unsafe { res.push_unchecked(log2) };
    n -= 1 << log2;

    let mut base = 3;
    let mut base_squared = 9;

    while n >= base_squared {
        let mut log = 1;
        let mut r = base;
        while r <= n / base {
            log += 1;
            r *= base;
        }

        unsafe { res.push_unchecked(log) };

        n -= r;
        base_squared += 2 * base + 1;
        base += 1;
    }
    (res, n)
}
