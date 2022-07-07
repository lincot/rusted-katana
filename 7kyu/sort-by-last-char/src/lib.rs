//! <https://www.codewars.com/kata/57eba158e8ca2c8aba0002a0/train/rust>

pub fn sort_by_last_char(s: &str) -> Vec<String> {
    let mut res = Vec::with_capacity(s.len() / 4);

    for word in s.as_bytes().split(|&b| b == b' ') {
        if !word.is_empty() {
            res.push(unsafe { String::from_utf8_unchecked(word.to_vec()) });
        }
    }

    res.sort_by_cached_key(|s| {
        if s.is_empty() {
            unsafe { std::hint::unreachable_unchecked() };
        }
        s.as_bytes()[s.len() - 1]
    });

    res
}
