//! <https://www.codewars.com/kata/5264d2b162488dc400000001/train/rust>

use unchecked_std::prelude::*;

pub fn spin_words(words: &str) -> String {
    let mut res = String::with_capacity(words.len() + 1);

    for word in words.split(' ') {
        unsafe {
            if has_at_least_n_chars(word, 5) {
                res.extend_unchecked(word.chars().rev());
            } else {
                res.push_str_unchecked(word);
            }
            res.push_unchecked(' ');
        }
    }
    res.pop();

    res
}

fn has_at_least_n_chars(s: &str, n: usize) -> bool {
    if s.len() < n {
        return false;
    }

    let mut count = 0;
    for _ in s.chars() {
        count += 1;
        if count == n {
            return true;
        }
    }

    false
}
