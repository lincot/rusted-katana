//! <https://www.codewars.com/kata/5286b2e162056fd0cb000c20/train/rust>

use digital::{MaxLenBase10, NumToString, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn collatz(mut n: u32) -> String {
    const NUMS_PER_LOOP: usize = 10;

    let mut res = n.to_string(false, false);

    loop {
        res.reserve(NUMS_PER_LOOP * (2 + u32::MAX_LEN_BASE10));

        for _ in 0..NUMS_PER_LOOP {
            if n == 1 {
                return res;
            }

            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };

            unsafe {
                res.as_mut_vec().push_unchecked(b'-');
                res.as_mut_vec().push_unchecked(b'>');
                res.write_num_unchecked(n, 10, false, false);
            }
        }
    }
}
