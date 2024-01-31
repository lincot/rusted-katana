//! <https://www.codewars.com/kata/5679aa472b8f57fb8c000047/train/rust>

pub fn find_even_index(arr: &[i32]) -> Option<usize> {
    let (mut l, mut r) = (0, arr.iter().sum::<i32>());
    #[allow(clippy::needless_range_loop)]
    for i in 0..arr.len() {
        r -= arr[i];
        if l == r {
            return Some(i);
        }
        l += arr[i];
    }
    None
}
