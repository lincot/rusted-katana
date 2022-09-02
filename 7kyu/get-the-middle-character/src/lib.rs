//! <https://www.codewars.com/kata/56747fd5cb988479af000028/train/rust>

use my_prelude::prelude::*;

pub fn get_middle(s: &str) -> &str {
    let mut char_indices = Vec::with_capacity(s.len());
    for (i, _) in s.char_indices() {
        unsafe { char_indices.push_unchecked(i) };
    }

    if char_indices.len() < 3 {
        s
    } else {
        unsafe {
            s.get_unchecked(
                *char_indices.get_unchecked((char_indices.len() - 1) / 2)
                    ..*char_indices.get_unchecked(char_indices.len() / 2 + 1),
            )
        }
    }
}
