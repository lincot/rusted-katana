//! <https://www.codewars.com/kata/5436f26c4e3d6c40e5000282/train/rust>

pub fn sum_of_n(n: i32) -> Vec<i32> {
    if n > 0 {
        (0..=n).map(|x| (x * x + x) / 2).collect()
    } else {
        (0..=-n).map(|x| -x * (x + 1) / 2).collect()
    }
}
