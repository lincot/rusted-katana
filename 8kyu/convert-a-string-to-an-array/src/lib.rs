//! <https://www.codewars.com/kata/57e76bc428d6fbc2d500036d/train/rust>

pub fn string_to_array(s: &str) -> Vec<String> {
    s.as_bytes()
        .split(|&b| b == b' ')
        .map(|word| unsafe { String::from_utf8_unchecked(word.to_vec()) })
        .collect()
}
