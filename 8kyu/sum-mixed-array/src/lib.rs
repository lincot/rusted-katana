//! <https://www.codewars.com/kata/57eaeb9578748ff92a000009/train/rust>

use either::Either;

pub fn sum_mix(arr: &[Either<i32, String>]) -> i32 {
    arr.iter()
        .map(|e| match e {
            Either::Left(n) => *n,
            Either::Right(s) => s.parse().unwrap(),
        })
        .sum()
}
