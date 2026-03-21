//! <https://www.codewars.com/kata/5a3dd29055519e23ec000074/train/rust>

pub fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    arr_a
        .iter()
        .zip(arr_b)
        .fold(0, |acc, pair| match pair {
            (correct, given) if correct == given => acc + 4,
            (_, &"") => acc,
            _ => acc - 1,
        })
        .max(0)
}
