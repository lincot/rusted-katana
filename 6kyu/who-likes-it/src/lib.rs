//! <https://www.codewars.com/kata/5266876b8f4bf2da9b000362/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn likes(names: &[&str]) -> String {
    match names {
        [] => "no one likes this".into(),
        [a] => {
            let mut res = String::with_capacity(a.len() + " likes this".len());
            unsafe {
                res.push_str_unchecked(a);
                res.push_str_unchecked(" likes this");
            }
            res
        }
        [a, b] => {
            let mut res = String::with_capacity(a.len() + b.len() + " and  like this".len());
            unsafe {
                res.push_str_unchecked(a);
                res.push_str_unchecked(" and ");
                res.push_str_unchecked(b);
                res.push_str_unchecked(" like this");
            }
            res
        }
        [a, b, c] => {
            let mut res =
                String::with_capacity(a.len() + b.len() + c.len() + ",  and  like this".len());
            unsafe {
                res.push_str_unchecked(a);
                res.push_str_unchecked(", ");
                res.push_str_unchecked(b);
                res.push_str_unchecked(" and ");
                res.push_str_unchecked(c);
                res.push_str_unchecked(" like this");
            }
            res
        }
        [a, b, rest @ ..] => {
            let mut res = String::with_capacity(
                a.len() + b.len() + ",  and  others like this".len() + usize::MAX_LEN_BASE10,
            );
            unsafe {
                res.push_str_unchecked(a);
                res.push_str_unchecked(", ");
                res.push_str_unchecked(b);
                res.push_str_unchecked(" and ");
                res.write_num_unchecked(rest.len(), 10, false, false);
                res.push_str_unchecked(" others like this");
            }
            res
        }
    }
}
