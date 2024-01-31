//! <https://www.codewars.com/kata/5513795bd3fafb56c200049e/train/rust>

use unchecked::PushUnchecked;

pub fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut cur = x;
    let mut res = Vec::with_capacity(n as _);
    for _ in 0..n {
        unsafe { res.push_unchecked(cur) };
        cur += x;
    }
    res
}
