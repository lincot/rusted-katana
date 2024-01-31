//! <https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9/train/rust>

pub fn find_short(s: &str) -> u32 {
    s.as_bytes()
        .split(|&b| b == b' ')
        .map(<[_]>::len)
        .min()
        .unwrap_or(0) as _
}
