//! <https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9/train/rust>

pub fn find_short(s: &str) -> u32 {
    s.split_ascii_whitespace().map(str::len).min().unwrap() as _
}
