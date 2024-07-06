//! <https://www.codewars.com/kata/59392ff00203d9686a0000c6/train/rust>

use unchecked_std::prelude::*;

pub fn guess<F>(f: F, n: usize) -> Vec<i32>
where
    F: Fn(usize, usize) -> i32,
{
    assert!(n >= 3);

    let mut res = Vec::with_capacity(n);

    let mut i = 0;
    while i + 2 < n {
        let s_0_1 = f(i, i + 1);
        let s_1_2 = f(i + 1, i + 2);
        let s_0_2 = f(i, i + 2);

        unsafe {
            res.push_unchecked((s_0_1 + s_0_2 - s_1_2) / 2);
            res.push_unchecked((s_1_2 + s_0_1 - s_0_2) / 2);
            res.push_unchecked((s_1_2 + s_0_2 - s_0_1) / 2);
        }
        i += 3;
    }

    if n > i {
        let j = n - 3;
        let s_0_1 = f(j, j + 1);
        let s_1_2 = f(j + 1, j + 2);
        let s_0_2 = f(j, j + 2);

        unsafe {
            if n == i + 2 {
                res.push_unchecked((s_1_2 + s_0_1 - s_0_2) / 2);
            }
            res.push_unchecked((s_1_2 + s_0_2 - s_0_1) / 2);
        }
    }

    res
}
