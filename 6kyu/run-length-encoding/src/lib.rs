//! <https://www.codewars.com/kata/546dba39fa8da224e8000467/train/rust>

use unchecked_core::PushUnchecked;

pub fn run_length_encoding(s: &str) -> Vec<(usize, char)> {
    let mut res = Vec::with_capacity(s.len());
    let mut chars = s.chars();
    let mut count = 1;
    let Some(mut prev) = chars.next() else {
        return res;
    };
    for c in chars {
        if c == prev {
            count += 1;
        } else {
            unsafe { res.push_unchecked((count, prev)) };
            (count, prev) = (1, c);
        }
    }
    unsafe { res.push_unchecked((count, prev)) };
    res
}
