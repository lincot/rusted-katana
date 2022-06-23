//! <https://www.codewars.com/kata/59ca8e8e1a68b7de740001f4/train/rust>

pub fn match_counts(a1: &[String], a2: &[String]) -> Vec<usize> {
    a2.iter()
        .map(|s2| a1.iter().filter(|&s1| s1 == s2).count())
        .collect()
}
