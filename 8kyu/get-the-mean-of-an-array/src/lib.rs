//! <https://www.codewars.com/kata/563e320cee5dddcf77000158/train/rust>

#![no_std]

pub fn get_average(marks: &[i32]) -> i32 {
    marks.iter().sum::<i32>() / marks.len() as i32
}
