//! <https://www.codewars.com/kata/590e03aef55cab099a0002e8/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn incrementer(nums: &[u32]) -> Vec<u32> {
    let mut res = Vec::with_capacity(nums.len());
    unsafe { res.set_len(nums.len()) };
    let mut res_ptr = res.as_mut_ptr();
    #[allow(clippy::needless_range_loop)]
    for i in 0..nums.len() {
        unsafe {
            *res_ptr = (i as u32 + 1 + nums[i]) % 10;
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
