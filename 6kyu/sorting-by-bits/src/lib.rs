//! <https://www.codewars.com/kata/59fa8e2646d8433ee200003f/train/rust>

pub fn sort_by_bit(arr: &mut [u32]) {
    vqsort_rs::sort(arr);
    arr.sort_by_key(|&x| x.count_ones());
}
