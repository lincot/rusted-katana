//! <https://www.codewars.com/kata/523a86aa4230ebb5420001e1/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use my_prelude::prelude::*;

pub fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let mut word_sorted = Vec::with_capacity(word.len());
    for c in word.chars() {
        unsafe { word_sorted.push_unchecked(c) };
    }
    word_sorted.sort_unstable();

    let mut res = Vec::with_capacity(words.len());

    for word2 in words {
        if word.len() != word2.len() {
            continue;
        }

        let mut word2_sorted = Vec::with_capacity(word2.len());
        for c in word2.chars() {
            unsafe { word2_sorted.push_unchecked(c) };
        }
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
