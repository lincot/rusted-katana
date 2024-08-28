//! <https://www.codewars.com/kata/5a8059b1fd577709860000f6/train/rust>

pub fn alphabetic(s: &str) -> bool {
    s.as_bytes().is_sorted()
}
