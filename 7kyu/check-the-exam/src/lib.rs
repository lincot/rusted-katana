//! <https://www.codewars.com/kata/5a3dd29055519e23ec000074/train/rust>

#![no_std]

pub fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    arr_a
        .iter()
        .zip(arr_b)
        .map(|(correct, given)| {
            if given == correct {
                4
            } else if given.is_empty() {
                0
            } else {
                -1
            }
        })
        .sum::<i64>()
        .max(0)
}
