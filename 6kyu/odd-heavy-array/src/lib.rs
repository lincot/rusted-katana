//! <https://www.codewars.com/kata/59c7e477dcc40500f50005c7/train/rust>

pub fn is_odd_heavy(arr: &[i32]) -> bool {
    let mut max_even = i32::MIN;
    let mut min_odd = i32::MAX;
    for &x in arr {
        if x % 2 == 0 {
            max_even = max_even.max(x);
        } else {
            min_odd = min_odd.min(x - 1);
        }
    }
    min_odd != i32::MAX && min_odd >= max_even
}
