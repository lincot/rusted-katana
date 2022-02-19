//! <https://www.codewars.com/kata/58f8b35fda19c0c79400020f/train/rust>

pub fn all_non_consecutive(arr: &[i32]) -> Vec<(usize, i32)> {
    // worst case capacity
    let mut res = Vec::with_capacity(arr.len());

    for i in 1..arr.len() {
        if arr[i - 1] + 1 != arr[i] {
            res.push((i, arr[i]));
        }
    }

    res
}
