//! <https://www.codewars.com/kata/577a98a6ae28071780000989/train/rust>

pub fn minimum(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

pub fn maximum(arr: &[i32]) -> i32 {
    *arr.iter().max().unwrap()
}
