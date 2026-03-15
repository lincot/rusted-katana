//! <https://www.codewars.com/kata/5f77d62851f6bc0033616bd8/train/rust>

pub fn valid_spacing(s: &str) -> bool {
    !(s.starts_with(' ') || s.ends_with(' ') || s.contains("  "))
}
