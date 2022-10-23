//! <https://www.codewars.com/kata/5245a9138ca049e9a10007b8/train/rust>

#![no_std]

pub fn count_adjacent_pairs(search_string: &str) -> usize {
    let mut res = 0;
    let mut prev_word: &[u8] = &[];
    let mut prev_eq = false;
    for word in search_string.as_bytes().split(|&b| b == b' ') {
        if !word.is_empty() && word.eq_ignore_ascii_case(prev_word) {
            if !prev_eq {
                res += 1;
            }
            prev_eq = true;
        } else {
            prev_eq = false;
        }
        prev_word = word;
    }
    res
}
