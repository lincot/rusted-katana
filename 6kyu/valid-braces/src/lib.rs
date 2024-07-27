//! <https://www.codewars.com/kata/5277c8a221e209d3f6000b56/train/rust>

use unchecked_std::prelude::*;

pub fn valid_braces(s: &str) -> bool {
    let mut stack = Vec::with_capacity(s.len());
    for b in s.bytes() {
        if [b'(', b'[', b'{'].contains(&b) {
            unsafe { stack.push_unchecked(b) };
        } else if stack
            .pop()
            .map_or(true, |last| ![b - 1, b - 2].contains(&last))
        {
            return false;
        }
    }
    stack.is_empty()
}
