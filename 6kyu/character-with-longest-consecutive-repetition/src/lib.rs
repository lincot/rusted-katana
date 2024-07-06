//! <https://www.codewars.com/kata/586d6cefbcc21eed7a001155/train/rust>

pub fn longest_repetition(s: &str) -> Option<(char, usize)> {
    let mut chars = s.chars();
    let mut prev_char = chars.next()?;
    let mut res = (prev_char, 1);
    let mut cur_count = 1;
    for c in chars {
        if c == prev_char {
            cur_count += 1;
        } else {
            if cur_count > res.1 {
                res = (prev_char, cur_count);
            }
            cur_count = 1;
            prev_char = c;
        }
    }
    Some(if cur_count > res.1 {
        (prev_char, cur_count)
    } else {
        res
    })
}
