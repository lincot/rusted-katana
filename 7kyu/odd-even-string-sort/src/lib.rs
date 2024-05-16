//! <https://www.codewars.com/kata/580755730b5a77650500010c/train/rust>

use unchecked_core::{PushStrUnchecked, PushUnchecked};

pub fn sort_my_string(s: &str) -> String {
    let mut res = String::with_capacity(s.len() + 1);
    let mut odd = String::with_capacity(s.len());
    unsafe {
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                res.push_unchecked(c);
            } else {
                odd.push_unchecked(c);
            }
        }
        res.push_unchecked(' ');
        res.push_str_unchecked(&odd);
    }
    res
}
