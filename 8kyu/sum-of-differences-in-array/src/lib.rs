//! <https://www.codewars.com/kata/5b73fe9fb3d9776fbf00009e/train/rust>

pub fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    if arr.len() < 2 {
        return None;
    }
    let first = arr[0];
    let (max, min) = arr.iter().fold((first, first), |(max, min), &next| {
        (max.max(next), min.min(next))
    });
    Some(max - min)
}
