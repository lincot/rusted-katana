//! <https://www.codewars.com/kata/5266876b8f4bf2da9b000362/train/rust>

use digital::prelude::*;
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
            let mut res = String::with_capacity(
                (a.len() + " and  like this".len())
                    .checked_add(b.len())
                    .unwrap(),
            );
            unsafe {
                res.push_str_unchecked(a);
                res.push_str_unchecked(" and ");
                res.push_str_unchecked(b);
                res.push_str_unchecked(" like this");
            }
            res
        }
        [a, b, c] => {
            let mut res = String::with_capacity(
                (a.len() + b.len())
                    .checked_add(c.len() + ",  and  like this".len())
                    .unwrap(),
            );
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
                (a.len() + ",  and  others like this".len() + usize::MAX_LEN_BASE10)
                    .checked_add(b.len())
                    .unwrap(),
            );
            unsafe {
                res.push_str_unchecked(a);
                res.push_str_unchecked(", ");
                res.push_str_unchecked(b);
                res.push_str_unchecked(" and ");
                res.write_int_unchecked(rest.len());
                res.push_str_unchecked(" others like this");
            }
            res
        }
    }
}
