//! <https://www.codewars.com/kata/57675f3dedc6f728ee000256/train/rust>

use core::mem::MaybeUninit;
use unchecked_std::prelude::*;

pub fn tower_builder(n_floors: usize, block_size: (usize, usize)) -> Vec<String> {
    assert!(block_size.0 != 0 && block_size.1 != 0);
    let mut res = Vec::with_capacity(n_floors * block_size.1);
    let floor_len = block_size.0 * (2 * n_floors).wrapping_sub(1);
    let mut border0 = block_size.0 * n_floors;
    let mut border1 = border0;

    for _ in 0..n_floors {
        let mut floor = Vec::with_capacity(floor_len);
        border0 -= block_size.0;
        let floor = unsafe {
            floor
                .spare_capacity_mut()
                .get_unchecked_mut(..border0)
                .fill(MaybeUninit::new(b' '));
            floor
                .spare_capacity_mut()
                .get_unchecked_mut(border0..border1)
                .fill(MaybeUninit::new(b'*'));
            floor
                .spare_capacity_mut()
                .get_unchecked_mut(border1..floor_len)
                .fill(MaybeUninit::new(b' '));
            floor.set_len(floor_len);
            String::from_utf8_unchecked(floor)
        };
        unsafe {
            for _ in 0..block_size.1 - 1 {
                res.push_unchecked(floor.clone());
            }
            res.push_unchecked(floor);
        }
        border1 = border1.wrapping_add(block_size.0);
    }

    res
}
