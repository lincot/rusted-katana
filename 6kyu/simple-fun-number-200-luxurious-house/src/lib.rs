//! <https://www.codewars.com/kata/58c8af49fd407dea5f000042/train/rust>

use core::mem::MaybeUninit;

pub fn luxhouse(houses: &[u32]) -> Vec<u32> {
    let mut res = Vec::with_capacity(houses.len());
    let mut max = 0;
    for (i, &x) in houses.iter().enumerate().rev() {
        let value = if x > max {
            max = x;
            0
        } else {
            max - x + 1
        };
        unsafe { *res.spare_capacity_mut().get_unchecked_mut(i) = MaybeUninit::new(value) };
    }
    unsafe { res.set_len(houses.len()) };
    res
}
