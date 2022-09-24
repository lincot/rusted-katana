//! <https://www.codewars.com/kata/57eb8fcdf670e99d9b000272/train/rust>

#![no_std]

pub fn high(input: &str) -> &str {
    let input_bytes = input.as_bytes();

    let mut highest_score = 0;
    let mut max_word_start = 0;
    let mut max_word_end = input.len();

    let mut cur_score = 0;
    let mut cur_word_start = 0;

    for (i, &b) in input_bytes.iter().enumerate() {
        if b == b' ' {
            if cur_score > highest_score {
                highest_score = cur_score;
                max_word_start = cur_word_start;
                max_word_end = i;
            }
            cur_score = 0;
            cur_word_start = i + 1;
        } else {
            cur_score += b - (b'a' - 1);
        }
    }

    if cur_score > highest_score {
        max_word_start = cur_word_start;
        max_word_end = input.len();
    }

    unsafe { input.get_unchecked(max_word_start..max_word_end) }
}
