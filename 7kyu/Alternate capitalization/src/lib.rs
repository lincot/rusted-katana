//! <https://www.codewars.com/kata/59cfc000aeb2844d16000075/train/rust>

use std::mem;

pub fn capitalize(s: &str) -> Vec<String> {
    // usually same capacity
    let mut first = String::with_capacity(s.len());
    let mut second = String::with_capacity(s.len());

    let mut chars_count = 0usize;

    for c in s.chars() {
        first.extend(c.to_uppercase());
        second.push(c);

        mem::swap(&mut first, &mut second);

        chars_count += 1;
    }

    if chars_count % 2 == 1 {
        mem::swap(&mut first, &mut second);
    }

    vec![first, second]
}
