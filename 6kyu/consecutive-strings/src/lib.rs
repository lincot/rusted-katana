//! <https://www.codewars.com/kata/56a5d994ac971f1ac500003e/train/rust>

use unchecked_std::prelude::*;

pub fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if strarr.len() < k || k == 0 {
        return String::new();
    }
    let mut len = 0;
    for s in &strarr[..k] {
        len = s.len().checked_add(len).unwrap();
    }
    let mut max_len = len;
    let mut best_i = 0;
    for i in k..strarr.len() {
        len = (len - unsafe { strarr.get_unchecked(i - k) }.len())
            .checked_add(strarr[i].len())
            .unwrap();
        if len > max_len {
            max_len = len;
            best_i = i - k + 1;
        }
    }

    let mut res = String::with_capacity(max_len);
    for s in unsafe { strarr.get_unchecked(best_i..best_i + k) } {
        unsafe { res.push_str_unchecked(s) };
    }
    res
}
