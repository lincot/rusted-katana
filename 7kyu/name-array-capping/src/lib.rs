//! <https://www.codewars.com/kata/5356ad2cbb858025d800111d/train/rust>

use unchecked_std::prelude::*;

pub fn cap_me(arr: Vec<String>) -> Vec<String> {
    arr.into_iter().map(capitalize_word).collect()
}

fn capitalize_word(word: String) -> String {
    if word.is_empty() {
        return word;
    }

    if word.as_bytes()[0].is_ascii() {
        return capitalize_word_likely_ascii(word);
    }

    capitalize_word_nonascii(word)
}

fn capitalize_word_likely_ascii(mut word: String) -> String {
    let bytes = unsafe { word.as_bytes_mut() };
    bytes[0] = bytes[0].to_ascii_uppercase();

    for b in &mut bytes[1..] {
        if b.is_ascii() {
            *b = b.to_ascii_lowercase();
        } else {
            return capitalize_word_nonascii(word);
        }
    }

    word
}

fn capitalize_word_nonascii(word: String) -> String {
    let mut res = String::with_capacity(3 + word.len() + word.len() / 2);
    unsafe {
        let mut word = word.chars();
        if let Some(ch) = word.next() {
            push_uppercase_unchecked(&mut res, ch);
        }

        for ch in word {
            push_lowercase_unchecked(&mut res, ch);
        }
    }
    res
}

unsafe fn push_uppercase_unchecked(s: &mut String, ch: char) {
    unsafe {
        if ch.is_ascii() {
            s.push_unchecked(ch.to_ascii_uppercase());
        } else if ch.is_uppercase() {
            s.push_unchecked(ch);
        } else {
            s.extend_unchecked(ch.to_uppercase());
        }
    }
}

unsafe fn push_lowercase_unchecked(s: &mut String, ch: char) {
    unsafe {
        if ch.is_ascii() {
            s.push_unchecked(ch.to_ascii_lowercase());
        } else if ch.is_lowercase() {
            s.push_unchecked(ch);
        } else {
            s.extend_unchecked(ch.to_lowercase());
        }
    }
}
