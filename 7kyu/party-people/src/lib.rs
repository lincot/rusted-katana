//! <https://www.codewars.com/kata/65013fc50038a68939098dcf/train/rust>

#![no_std]

extern crate alloc;
use vqsort::VqSort;

pub fn party_people(lst: &[u32]) -> u32 {
    let mut lst = lst.to_vec().into_boxed_slice();
    if lst.len() <= 20 {
        lst.sort_unstable();
    } else {
        VqSort::sort_ascending(&mut lst);
    }
    lst.iter()
        .enumerate()
        .rposition(|(i, &x)| x <= (i + 1) as u32)
        .map_or(0, |i| (i + 1) as u32)
}
