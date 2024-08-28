//! <https://www.codewars.com/kata/5b16490986b6d336c900007d/train/rust>

use std::collections::HashMap;
use unchecked_std::prelude::*;

pub fn my_languages(results: HashMap<&str, i32>) -> Vec<&str> {
    const MAX_SCORE: usize = 100;
    const MIN_SCORE: usize = 60;

    let mut top_languages = [None; MAX_SCORE - MIN_SCORE + 1];
    for (&lang, &score) in &results {
        if score >= MIN_SCORE as i32 {
            top_languages[score as usize - MIN_SCORE] = Some(lang);
        }
    }

    let mut res = Vec::with_capacity(top_languages.len());
    for &lang in top_languages.iter().rev() {
        if let Some(lang) = lang {
            unsafe { res.push_unchecked(lang) };
        }
    }
    res
}
