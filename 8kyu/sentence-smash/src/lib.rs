//! <https://www.codewars.com/kata/53dc23c68a0c93699800041d/train/rust>

use unchecked_std::prelude::*;

pub fn smash(words: &[&str]) -> String {
    if words.is_empty() {
        return String::new();
    }
    let len = words.iter().fold(words.len() - 1, |acc, word| {
        acc.checked_add(word.len()).unwrap()
    });
    let mut res = String::with_capacity(len);
    unsafe {
        res.push_str_unchecked(words[0]);
        for word in &words[1..] {
            res.push_unchecked(' ');
            res.push_str_unchecked(word);
        }
    }
    res
}
