//! <https://www.codewars.com/kata/56e3cd1d93c3d940e50006a4/train/rust>

pub fn make_valley(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr.to_vec();
    arr.sort_unstable_by_key(|&v| std::cmp::Reverse(v));

    let len = arr.len();
    let mut res = vec![0; len];

    for i in 0..len / 2 {
        unsafe {
            let left = res.get_unchecked_mut(i);
            *left = *arr.get_unchecked(i * 2);
            let right = res.get_unchecked_mut(len - 1 - i);
            *right = *arr.get_unchecked(i * 2 + 1);
        }
    }

    if len % 2 == 1 {
        unsafe {
            let mid = res.get_unchecked_mut(len / 2);
            *mid = *arr.get_unchecked(len - 1);
        }
    }

    res
}
