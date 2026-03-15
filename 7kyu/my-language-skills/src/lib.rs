//! <https://www.codewars.com/kata/5b16490986b6d336c900007d/train/rust>

use core::cmp::Reverse;
use std::collections::HashMap;
use unchecked_std::prelude::*;

pub fn my_languages(results: HashMap<&str, i32>) -> Vec<&str> {
    let mut top_languages = Vec::with_capacity(results.len());
    for (lang, score) in results {
        if score >= 60 {
            unsafe { top_languages.push_unchecked((lang, score as usize)) };
        }
    }
    top_languages.sort_unstable_by_key(|&(_, score)| Reverse(score));
    top_languages.iter().map(|&(lang, _)| lang).collect()
}
