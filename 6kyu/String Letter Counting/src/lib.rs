//! <https://www.codewars.com/kata/59e19a747905df23cb000024/train/rust>

use std::io::Write;

pub fn string_letter_count(s: &str) -> String {
    let mut counts = [0usize; 26];

    for b in s.bytes() {
        if (b'a'..=b'z').contains(&b) {
            counts[(b - b'a') as usize] += 1;
        } else if (b'A'..=b'Z').contains(&b) {
            counts[(b - b'A') as usize] += 1;
        }
    }

    let mut res = Vec::with_capacity(9 * 26);

    for (letter, count) in (b'a'..).zip(counts) {
        if count != 0 {
            write!(res, "{}", count).unwrap();
            res.push(letter);
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}
