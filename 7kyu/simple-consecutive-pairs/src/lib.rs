//! <https://www.codewars.com/kata/5a3e1319b6486ac96f000049/train/rust>

pub fn pairs(arr: &[i32]) -> usize {
    arr.chunks_exact(2)
        .filter(|chunk| (chunk[0] - chunk[1]).abs() == 1)
        .count()
}
