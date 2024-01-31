//! <https://www.codewars.com/kata/59f08f89a5e129c543000069/train/rust>

use unchecked::PushUnchecked;

pub fn dup(mut arry: Vec<String>) -> Vec<String> {
    for s in &mut arry {
        let mut res = String::with_capacity(s.len());

        let Some(mut prev) = s.chars().next() else {
            continue;
        };
        unsafe { res.push_unchecked(prev) };
        for c in s.chars() {
            if c != prev {
                unsafe { res.push_unchecked(c) };
            }
            prev = c;
        }

        *s = res;
    }

    arry
}
