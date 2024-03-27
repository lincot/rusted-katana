//! <https://www.codewars.com/kata/59cfc000aeb2844d16000075/train/rust>

use core::mem::swap;
use unchecked_core::{ExtendUnchecked, PushUnchecked};

pub fn capitalize(s: &str) -> Vec<String> {
    let cap = 2 * s.len() + s.len() / 3;
    let mut first = String::with_capacity(cap);
    let mut second = String::with_capacity(cap);

    let mut chars_count = 0usize;

    for c in s.chars() {
        unsafe {
            first.extend_unchecked(c.to_uppercase());
            second.push_unchecked(c);
        }

        swap(&mut first, &mut second);

        chars_count += 1;
    }
    if chars_count % 2 == 1 {
        swap(&mut first, &mut second);
    }

    vec![first, second]
}
