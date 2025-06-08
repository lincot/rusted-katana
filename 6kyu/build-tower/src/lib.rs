//! <https://www.codewars.com/kata/576757b1df89ecf5bd00073b/train/rust>

use core::mem::MaybeUninit;

pub fn tower_builder(n_floors: usize) -> Vec<String> {
    let width = n_floors.checked_mul(2).unwrap().wrapping_sub(1);
    let mut border0 = n_floors;
    let mut border1 = border0;
    (0..n_floors)
        .map(|_| {
            let mut floor = Vec::with_capacity(width);
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
                    .get_unchecked_mut(border1..width)
                    .fill(MaybeUninit::new(b' '));
                floor.set_len(width);
                border1 = border1.wrapping_add(1);
                String::from_utf8_unchecked(floor)
            }
        })
        .collect()
}
