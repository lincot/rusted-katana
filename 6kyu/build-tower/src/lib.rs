//! <https://www.codewars.com/kata/576757b1df89ecf5bd00073b/train/rust>

use core::mem::MaybeUninit;

pub fn tower_builder(n_floors: usize) -> Vec<String> {
    let mut border0 = n_floors;
    let mut border1 = border0;
    (0..n_floors)
        .map(|_| {
            let len = 2 * n_floors - 1;
            let mut floor = Vec::with_capacity(len);
            border0 -= 1;
            unsafe {
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
                    .get_unchecked_mut(border1..len)
                    .fill(MaybeUninit::new(b' '));
                floor.set_len(len);
                border1 = border1.wrapping_add(1);
                String::from_utf8_unchecked(floor)
            }
        })
        .collect()
}
