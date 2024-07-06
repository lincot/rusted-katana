//! <https://www.codewars.com/kata/576757b1df89ecf5bd00073b/train/rust>

use core::mem::MaybeUninit;

pub fn tower_builder(n_floors: usize) -> Vec<String> {
    (0..n_floors)
        .map(|i| {
            let len = 2 * n_floors - 1;
            let mut floor = Vec::with_capacity(len);
            let border0 = n_floors - i - 1;
            let border1 = border0 + 2 * i + 1;
            let border2 = border1 + n_floors - i - 1;
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
                    .get_unchecked_mut(border1..border2)
                    .fill(MaybeUninit::new(b' '));
                floor.set_len(len);
                String::from_utf8_unchecked(floor)
            }
        })
        .collect()
}
