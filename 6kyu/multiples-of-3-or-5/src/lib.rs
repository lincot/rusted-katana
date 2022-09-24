//! <https://www.codewars.com/kata/514b92a657cdc65150000006/train/rust>

#![no_std]

pub const fn solution(num: i32) -> i32 {
    if num <= 3 {
        return 0;
    }
    let num = num - 1;
    sum_of_sequence(3, num) + sum_of_sequence(5, num) - sum_of_sequence(15, num)
}

const fn sum_of_sequence(step: i32, stop_inclusive: i32) -> i32 {
    let n = stop_inclusive / step;
    let s = (n * (n + 1)) / 2;
    step * s
}
