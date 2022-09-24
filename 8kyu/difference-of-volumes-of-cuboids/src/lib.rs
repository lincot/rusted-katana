//! <https://www.codewars.com/kata/58cb43f4256836ed95000f97/train/rust>

#![no_std]

pub fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    (a.iter().product::<i32>() - b.iter().product::<i32>()).abs()
}
