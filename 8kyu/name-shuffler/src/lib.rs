//! <https://www.codewars.com/kata/559ac78160f0be07c200005a/train/rust>

use unchecked_std::prelude::*;

pub fn name_shuffler(s: &str) -> String {
    let mut res = String::with_capacity(s.len());

    let first_len = s.as_bytes().iter().position(|&b| b == b' ').unwrap();

    unsafe {
        res.push_str_unchecked(s.get_unchecked(first_len + 1..));
        res.as_mut_vec().push_unchecked(b' ');
        res.push_str_unchecked(s.get_unchecked(..first_len));
    }

    res
}
