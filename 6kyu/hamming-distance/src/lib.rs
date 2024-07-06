//! <https://www.codewars.com/kata/5410c0e6a0e736cf5b000e69/train/rust>

pub fn hamming(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
}
