//! <https://www.codewars.com/kata/515f51d438015969f7000013/train/rust>

pub fn pyramid(n: usize) -> Vec<Vec<u32>> {
    (1..n + 1).map(|len| vec![1; len]).collect()
}
