//! <https://www.codewars.com/kata/5aec1ed7de4c7f3517000079/train/rust>

pub fn first_n_smallest(arr: &[i32], n: usize) -> Vec<i32> {
    let mut arr_enumerated: Box<[_]> = arr.iter().copied().zip(0usize..).collect();
    arr_enumerated.select_nth_unstable(n.saturating_sub(1));
    let first_n = unsafe { arr_enumerated.get_unchecked_mut(..n) };
    if n < 64 {
        first_n.sort_by_key(|&(_, i)| i);
    } else {
        radsort::sort_by_key(first_n, |&(_, i)| i);
    }
    first_n.iter().map(|&(x, _)| x).collect()
}
