//! <https://www.codewars.com/kata/581331293788bc1702001fa6/train/rust>

use unchecked_std::prelude::*;

pub fn mirror(text: &str) -> String {
    let mut len_sum = 0;
    let mut utf8_len_sum = 0;
    let mut max_utf8_len = 0;
    let mut words_count = 0;
    for word in text.as_bytes().split(|&b| b == b' ') {
        let utf8_len = unsafe { core::str::from_utf8_unchecked(word).chars().count() };
        max_utf8_len = max_utf8_len.max(utf8_len);
        utf8_len_sum += utf8_len;
        words_count += 1;
        len_sum += word.len();
    }

    let capacity = (words_count + 2) * (max_utf8_len + 5) + len_sum - utf8_len_sum - 1;
    let mut res = String::with_capacity(capacity);

    unsafe {
        for _ in 0..max_utf8_len + 4 {
            res.push_unchecked('*');
        }
        for word in text.as_bytes().split(|&b| b == b' ') {
            let word = core::str::from_utf8_unchecked(word);
            res.push_str_unchecked("\n* ");
            let mut len = 0;
            for ch in word.chars().rev() {
                res.push_unchecked(ch);
                len += 1;
            }
            for _ in 0..max_utf8_len - len {
                res.push_unchecked(' ');
            }
            res.push_str_unchecked(" *");
        }
        res.push_unchecked('\n');
        for _ in 0..max_utf8_len + 4 {
            res.push_unchecked('*');
        }

        if res.len() != capacity {
            core::hint::unreachable_unchecked();
        }
    }

    res
}
