//! <https://www.codewars.com/kata/550554fd08b86f84fe000a58/train/rust>

use unchecked_std::prelude::*;

pub fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut res = Vec::with_capacity(arr_a.len());
    for &a in arr_a {
        if arr_b.iter().any(|&b| b.contains(a)) {
            unsafe { res.push_unchecked(a.into()) };
        }
    }
    res.sort_unstable();
    res.dedup();
    res
}
