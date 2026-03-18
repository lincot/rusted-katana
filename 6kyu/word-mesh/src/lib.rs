//! <https://www.codewars.com/kata/5c1ae703ba76f438530000a2/train/rust>

use unchecked_std::prelude::*;

pub fn word_mesh(words: &[&str]) -> Option<String> {
    let mut res = String::with_capacity(if words.is_empty() {
        0
    } else {
        let mut sum = 0;
        for word in &words[..words.len() - 1] {
            sum = word.len().checked_add(sum).unwrap();
        }
        sum
    });

    'pairs: for &[left, right] in words.array_windows() {
        for i in left.len().saturating_sub(right.len())..left.len() {
            if let Some(suffix) = left.get(i..)
                && right.starts_with(suffix)
            {
                unsafe { res.push_str_unchecked(suffix) };
                continue 'pairs;
            }
        }

        return None;
    }

    Some(res)
}
