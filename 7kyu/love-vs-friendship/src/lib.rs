//! <https://www.codewars.com/kata/59706036f6e5d1e22d000016/train/rust>

pub fn words_to_marks(s: &str) -> u32 {
    s.as_bytes().iter().map(|&b| b as u32).sum::<u32>() - (b'a' - 1) as u32 * s.len() as u32
}
