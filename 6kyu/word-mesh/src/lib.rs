//! <https://www.codewars.com/kata/5c1ae703ba76f438530000a2/train/rust>

use my_prelude::prelude::*;

pub fn word_mesh(words: &[&str]) -> Option<String> {
    let mut res = String::with_capacity(if words.is_empty() {
        0
    } else {
        words[..words.len() - 1].iter().map(|s| s.len()).sum()
    });

    'pairs: for pair in words.windows(2) {
        let left = pair[0];
        let right = pair[1];

        for i in left.len().saturating_sub(right.len())..left.len() {
            if let Some(suffix) = left.get(i..) {
                if right.starts_with(suffix) {
                    unsafe { res.push_str_unchecked(suffix) };
                    continue 'pairs;
                }
            }
        }

        return None;
    }

    Some(res)
}
