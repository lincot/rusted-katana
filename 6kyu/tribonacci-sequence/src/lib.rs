//! <https://www.codewars.com/kata/556deca17c58da83c00002db/train/rust>

use core::hint::unreachable_unchecked;
use unchecked_std::prelude::*;

pub fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    let mut res = Vec::with_capacity(n);
    unsafe { res.extend_from_slice_unchecked(&signature[..n.min(3)]) };
    for i in 3..n {
        if i != res.len() || res.len() < 3 {
            unsafe { unreachable_unchecked() };
        }
        unsafe { res.push_unchecked(res[i - 3..].iter().sum()) };
    }
    res
}
