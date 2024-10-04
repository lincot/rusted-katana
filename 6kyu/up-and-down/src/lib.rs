//! <https://www.codewars.com/kata/56cac350145912e68b0006f0/train/rust>

use unchecked_std::prelude::*;

pub fn arrange(s: &str) -> String {
    let mut res = String::with_capacity(3 * s.len());

    let mut words = s
        .as_bytes()
        .split(|&b| b == b' ')
        .map(|s| unsafe { core::str::from_utf8_unchecked(s) });

    let Some(mut prev_prev) = words.next() else {
        return res;
    };

    unsafe {
        let Some(mut prev) = words.next() else {
            push_str_lowercase_unchecked(&mut res, prev_prev);
            return res;
        };
        let Some(mut curr) = words.next() else {
            if prev_prev > prev {
                (prev, prev_prev) = (prev_prev, prev);
            }
            push_str_lowercase_unchecked(&mut res, prev_prev);
            res.push_unchecked(' ');
            push_str_uppercase_unchecked(&mut res, prev);
            return res;
        };

        loop {
            if prev_prev.len() > prev.len() {
                (prev, prev_prev) = (prev_prev, prev);
            }
            push_str_lowercase_unchecked(&mut res, prev_prev);
            res.push_unchecked(' ');
            if prev.len() < curr.len() {
                (prev, curr) = (curr, prev);
            }
            push_str_uppercase_unchecked(&mut res, prev);
            res.push_unchecked(' ');

            prev_prev = curr;
            let Some(prev_) = words.next() else {
                push_str_lowercase_unchecked(&mut res, curr);
                return res;
            };
            prev = prev_;
            let Some(curr_) = words.next() else {
                if prev_prev.len() > prev.len() {
                    (prev, prev_prev) = (prev_prev, prev);
                }
                push_str_lowercase_unchecked(&mut res, prev_prev);
                res.push_unchecked(' ');
                push_str_uppercase_unchecked(&mut res, prev);

                return res;
            };
            curr = curr_;
        }
    }
}

unsafe fn push_str_uppercase_unchecked(s: &mut String, string_to_push: &str) {
    for ch in string_to_push.chars() {
        push_uppercase_unchecked(s, ch);
    }
}

unsafe fn push_uppercase_unchecked(s: &mut String, ch: char) {
    if ch.is_ascii() {
        s.push_unchecked(ch.to_ascii_uppercase());
    } else {
        s.extend_unchecked(ch.to_uppercase());
    }
}

unsafe fn push_str_lowercase_unchecked(s: &mut String, string_to_push: &str) {
    for ch in string_to_push.chars() {
        push_lowercase_unchecked(s, ch);
    }
}

unsafe fn push_lowercase_unchecked(s: &mut String, ch: char) {
    if ch.is_ascii() {
        s.push_unchecked(ch.to_ascii_lowercase());
    } else if ch.is_lowercase() {
        s.push_unchecked(ch);
    } else {
        s.extend_unchecked(ch.to_lowercase());
    }
}
