//! <https://www.codewars.com/kata/57356c55867b9b7a60000bd7/train/rust>

#![no_std]

pub const fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
    match operator {
        '+' => value1 + value2,
        '-' => value1 - value2,
        '*' => value1 * value2,
        _ => value1 / value2,
    }
}
