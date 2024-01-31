//! <https://www.codewars.com/kata/5769b3802ae6f8e4890009d2/train/rust>

pub fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    (0..(arr.len() + 1) / 2)
        .map(|i| unsafe { *arr.get_unchecked(2 * i) })
        .collect()
}
