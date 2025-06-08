//! <https://www.codewars.com/kata/5526fc09a1bbd946250002dc/train/rust>

pub fn find_outlier(arr: &[i32]) -> i32 {
    assert!(arr.len() >= 3);
    let rem = ((arr[0] & 1) + (arr[1] & 1) + (arr[2] & 1)) >> 1;
    *arr.iter().find(|&x| x & 1 != rem).unwrap()
}
