//! <https://www.codewars.com/kata/523a86aa4230ebb5420001e1/train/rust>

use my_prelude::prelude::*;

pub fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let mut word_sorted: Vec<_> = word.chars().collect();
    word_sorted.sort_unstable();

    let mut res = Vec::with_capacity(words.len());

    for word2 in words {
        if word.len() != word2.len() {
            continue;
        }

        let mut word2_sorted: Vec<_> = word2.chars().collect();
        if word_sorted.len() != word2_sorted.len() {
            continue;
        }
        word2_sorted.sort_unstable();

        if word_sorted == word2_sorted {
            unsafe { res.push_unchecked(word2.clone()) };
        }
    }

    res
}
