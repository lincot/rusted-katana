//! <https://www.codewars.com/kata/5436f26c4e3d6c40e5000282/train/rust>

pub fn sum_of_n(n: i32) -> Vec<i32> {
    let sign = n.signum();
    (0..n.abs() + 1).map(|x| sign * (x * (x + 1)) / 2).collect()
}
