//! <https://www.codewars.com/kata/5a3e1319b6486ac96f000049/train/rust>

pub fn pairs(arr: &[i32]) -> usize {
    arr.as_chunks::<2>()
        .0
        .iter()
        .filter(|[a, b]| (a - b).abs() == 1)
        .count()
}
