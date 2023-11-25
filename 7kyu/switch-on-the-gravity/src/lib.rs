//! <https://www.codewars.com/kata/64c743cb0a2a00002856ff73/train/rust>

#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};

pub fn switch_gravity(lst: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut air = vec![lst.len(); lst[0].len()];
    for row in lst {
        assert!(air.len() == row.len());
        air.iter_mut()
            .zip(row)
            .for_each(|(a, r)| *a -= (*r == '#') as usize);
    }

    let mut res: Vec<_> = (0..lst.len()).map(|_| vec!['-'; air.len()]).collect();
    for (j, &height) in air.iter().enumerate() {
        for i in height..res.len() {
            unsafe { *res.get_unchecked_mut(i).get_unchecked_mut(j) = '#' };
        }
    }
    res
}
