//! <https://www.codewars.com/kata/58c8af49fd407dea5f000042/train/rust>

use core::mem::MaybeUninit;

pub fn luxhouse(houses: &[u32]) -> Vec<u32> {
    let mut res = Vec::with_capacity(houses.len());
    let mut ptr = res
        .spare_capacity_mut()
        .as_mut_ptr()
        .wrapping_add(houses.len());
    let mut max = 0;
    for &x in houses.iter().rev() {
        ptr = ptr.wrapping_sub(1);
        // somehow replacing `x > max` with `max < x` makes it 30% faster
        let value = if max < x {
            max = x;
            0
        } else {
            max - x + 1
        };
        unsafe { ptr.write(MaybeUninit::new(value)) };
    }
    unsafe { res.set_len(houses.len()) };
    res
}
