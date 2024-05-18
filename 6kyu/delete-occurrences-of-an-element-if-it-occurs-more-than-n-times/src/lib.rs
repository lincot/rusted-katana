//! <https://www.codewars.com/kata/554ca54ffa7d91b236000023/train/rust>

use unchecked_std::prelude::*;

pub fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut counts = [0; u8::MAX as usize + 1];
    let mut res = Vec::with_capacity(lst.len());
    for &x in lst {
        counts[x as usize] += 1;
        if counts[x as usize] <= n {
            unsafe { res.push_unchecked(x) };
        }
    }
    res
}
