//! <https://www.codewars.com/kata/5a3e1319b6486ac96f000049/train/rust>

#![feature(array_chunks)]

pub fn pairs(arr: &[i32]) -> usize {
    arr.array_chunks()
        .filter(|[a, b]| (a - b).abs() == 1)
        .count()
}
