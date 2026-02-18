//! <https://www.codewars.com/kata/55c45be3b2079eccff00010f/train/rust>

use unchecked_std::prelude::*;

const MAX_WORDS: usize = 10;

pub fn order(sentence: &str) -> String {
    let mut res = String::with_capacity(sentence.len());
    let mut words = heapless::Vec::<_, MAX_WORDS>::new();
    'outer: for word in sentence
        .as_bytes()
        .split(|&b| b == b' ')
        .map(|s| unsafe { core::str::from_utf8_unchecked(s) })
    {
        for b in word.bytes() {
            if b.is_ascii_digit() {
                unsafe { words.push_unchecked((b, word)) };

                if words.len() == MAX_WORDS {
                    break 'outer;
                }

                break;
            }
        }
    }
    if words.len() > MAX_WORDS {
        unsafe { core::hint::unreachable_unchecked() };
    }
    words.sort_unstable_by_key(|&(n, _)| n);
    let Some((_, first_word)) = words.first() else {
        return res;
    };
    unsafe { res.push_str_unchecked(first_word) };
    for (_, word) in &words[1..] {
        unsafe {
            res.push_unchecked(' ');
            res.push_str_unchecked(word);
        }
    }
    res
}
