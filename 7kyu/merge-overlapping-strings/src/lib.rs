//! <https://www.codewars.com/kata/61c78b57ee4be50035d28d42/train/rust>

use core::hint::unreachable_unchecked;
use unchecked_std::prelude::*;

pub fn merge_strings(first: &str, second: &str) -> String {
    unsafe {
        for len in (0..first.len().min(second.len()) + 1).rev() {
            if first.get_unchecked(first.len() - len..) == second.get_unchecked(..len) {
                let mut res = String::with_capacity(first.len() + second.len() - len);
                res.push_str_unchecked(first);
                res.push_str_unchecked(second.get_unchecked(len..));
                return res;
            }
        }
        unreachable_unchecked()
    }
}
