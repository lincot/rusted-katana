//! <https://www.codewars.com/kata/586d6cefbcc21eed7a001155/train/rust>

pub fn longest_repetition(s: &str) -> Option<(char, usize)> {
    if s.is_ascii() {
        return longest_repetition_bytes(s.as_bytes()).map(|(ch, count)| (ch as _, count));
    }

    let mut chars = s.chars();
    let mut prev_char = chars.next()?;
    let mut res = (prev_char, 1);
    let mut cur_count = 1;
    for ch in chars {
        if ch == prev_char {
            cur_count += 1;
        } else {
            if cur_count > res.1 {
                res = (prev_char, cur_count);
            }
            cur_count = 1;
            prev_char = ch;
        }
    }
    Some(if cur_count > res.1 {
        (prev_char, cur_count)
    } else {
        res
    })
}

fn longest_repetition_bytes(s: &[u8]) -> Option<(u8, usize)> {
    let mut bytes = s.iter();
    let mut prev_char = *bytes.next()?;
    let mut res = (prev_char as _, 1);
    let mut cur_count = 1;
    for &b in bytes {
        if b == prev_char {
            cur_count += 1;
        } else {
            if cur_count > res.1 {
                res = (prev_char as _, cur_count);
            }
            cur_count = 1;
            prev_char = b;
        }
    }
    Some(if cur_count > res.1 {
        (prev_char as _, cur_count)
    } else {
        res
    })
}
