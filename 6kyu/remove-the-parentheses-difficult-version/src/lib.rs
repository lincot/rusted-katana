//! <https://www.codewars.com/kata/62f39741ec164f2b28c257c5/train/rust>

use unchecked::{PushStrUnchecked, PushUnchecked};

pub fn remove_parentheses(s: &str) -> String {
    let mut res = String::with_capacity(s.len());

    let mut unclosed_opening_parentheses = Vec::with_capacity(s.len());
    let mut closed_pairs = Vec::with_capacity(s.len());

    for (i, b) in s.bytes().enumerate() {
        if b == b'(' {
            unsafe { unclosed_opening_parentheses.push_unchecked(i) };
        } else if b == b')' {
            if let Some(a) = unclosed_opening_parentheses.pop() {
                unsafe { closed_pairs.push_unchecked((a, i)) };
            }
        }
    }

    closed_pairs.sort_unstable_by_key(|&(a, _)| a);

    let mut start = 0;
    for (a, b) in closed_pairs {
        if a > start {
            unsafe { res.push_str_unchecked(s.get_unchecked(start..a)) };
        }
        start = start.max(b + 1);
    }
    unsafe { res.push_str_unchecked(s.get_unchecked(start..)) };

    res
}
