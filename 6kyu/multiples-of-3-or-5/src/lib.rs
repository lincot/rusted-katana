//! <https://www.codewars.com/kata/514b92a657cdc65150000006/train/rust>

pub const fn solution(num: i32) -> i32 {
    #[inline]
    const fn s(step: i32, stop: i32) -> i32 {
        let n = (stop - 1) / step;
        step * n * (n + 1)
    }

    if num <= 3 {
        0
    } else {
        (s(3, num) + s(5, num) - s(15, num)) / 2
    }
}
