//! <https://www.codewars.com/kata/59df2f8f08c6cec835000012/train/rust>

use char_to_lower::to_lower;
use core::cmp::Ordering;
use unchecked_std::prelude::*;

pub fn meeting(s: &str) -> String {
    let mut names = Vec::with_capacity(s.len() * 2 / 3);
    for full_name in s
        .as_bytes()
        .split(|&b| b == b';')
        .map(|s| unsafe { core::str::from_utf8_unchecked(s) })
    {
        let full_name = full_name.split_once(':').unwrap();
        unsafe { names.push_unchecked((CaselessStr(full_name.1), CaselessStr(full_name.0))) };
    }

    names.sort_unstable();

    let mut res = String::with_capacity(s.len() * 3);
    for (last_name, first_name) in names {
        unsafe {
            res.push_unchecked('(');
            push_str_uppercase_unchecked(&mut res, last_name.0);
            res.push_str_unchecked(", ");
            push_str_uppercase_unchecked(&mut res, first_name.0);
            res.push_unchecked(')');
        }
    }
    res
}

#[derive(Clone, Copy)]
struct CaselessStr<'a>(&'a str);

impl PartialEq for CaselessStr<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for CaselessStr<'_> {}

impl Ord for CaselessStr<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        for (ch1, ch2) in self
            .0
            .chars()
            .map(to_lower)
            .zip(other.0.chars().map(to_lower))
        {
            let ord = ch1.cmp(&ch2);
            if ord != Ordering::Equal {
                return ord;
            }
        }
        self.0.len().cmp(&other.0.len())
    }
}

impl PartialOrd for CaselessStr<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

unsafe fn push_str_uppercase_unchecked(s: &mut String, string_to_push: &str) {
    for ch in string_to_push.chars() {
        push_uppercase_unchecked(s, ch);
    }
}

unsafe fn push_uppercase_unchecked(s: &mut String, ch: char) {
    if ch.is_ascii() {
        s.push_unchecked(ch.to_ascii_uppercase());
    } else {
        s.extend_unchecked(ch.to_uppercase());
    }
}
