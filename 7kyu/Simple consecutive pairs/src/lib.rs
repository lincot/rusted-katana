//! <https://www.codewars.com/kata/5a3e1319b6486ac96f000049/train/rust>

pub fn pairs(arr: &[i32]) -> usize {
    arr.chunks_exact(2)
        .filter(|pair| (pair[0] - pair[1]).abs() == 1)
        .count()
}
