//! <https://www.codewars.com/kata/59e19a747905df23cb000024/train/rust>

use my_prelude::prelude::*;

pub fn string_letter_count(s: &str) -> String {
    let mut counts = [0usize; 26];

    for b in s.bytes() {
        if (b'a'..=b'z').contains(&b) {
            counts[(b - b'a') as usize] += 1;
        } else if (b'A'..=b'Z').contains(&b) {
            counts[(b - b'A') as usize] += 1;
        }
    }

    let mut res = String::with_capacity((18 + 1) * 26);

    for (letter, count) in (b'a'..).zip(counts) {
        if count != 0 {
            unsafe {
                res.write_num_unchecked(count);
                res.as_mut_vec().push_unchecked(letter);
            }
        }
    }

    res
}
