//! <https://www.codewars.com/kata/526233aefd4764272800036f/train/rust>

pub fn matrix_addition(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
    a.iter()
        .zip(b)
        .map(|(a, b)| a.iter().zip(b).map(|(a, b)| a + b).collect())
        .collect()
}
