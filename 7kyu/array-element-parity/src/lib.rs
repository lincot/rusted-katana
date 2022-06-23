//! <https://www.codewars.com/kata/5a092d9e46d843b9db000064/train/rust>

pub fn solve(arr: &[i32]) -> i32 {
    arr.iter().sum::<i32>()
        / arr
            .iter()
            .map(|&x| if x > 0 { 1 } else { -1 })
            .sum::<i32>()
            .abs()
}
