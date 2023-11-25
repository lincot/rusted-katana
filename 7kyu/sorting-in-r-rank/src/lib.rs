//! <https://www.codewars.com/kata/65006177f534f65b2594df05/train/rust>

#![no_std]

extern crate alloc;
use alloc::{boxed::Box, vec::Vec};

pub fn rank<T: Copy + Ord>(lst: &[T]) -> Vec<f64> {
    let mut lst_indexed: Box<[_]> = lst.iter().copied().enumerate().collect();
    if lst.len() != lst_indexed.len() {
        unsafe { core::hint::unreachable_unchecked() };
    }
    lst_indexed.sort_unstable_by_key(|&(_, x)| x);
    let mut res = Vec::with_capacity(lst.len());
    unsafe { res.set_len(lst.len()) };
    let mut i = 0;
    while i < lst.len() {
        let next_different_i = lst_indexed[i..]
            .iter()
            .position(|x| x.1 != lst_indexed[i].1)
            .map_or(lst.len(), |j| j + i);
        let rank = ((i + next_different_i - 1) / 2) as f64
            + if (next_different_i - i) % 2 == 0 {
                0.5
            } else {
                0.
            };
        unsafe {
            for &(j, _) in lst_indexed.get_unchecked(i..next_different_i) {
                *res.get_unchecked_mut(j) = rank;
            }
        }
        i = next_different_i;
    }
    res
}
