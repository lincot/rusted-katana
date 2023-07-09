//! <https://www.codewars.com/kata/5a91a7c5fd8c061367000002/train/rust>

#![no_std]

extern crate alloc;

pub fn minimum_steps(nums: &[i32], value: i32) -> usize {
    let mut nums = nums.to_vec();
    if nums.len() < 160 {
        nums.sort_unstable();
    } else {
        radsort::sort(&mut nums);
    }

    let mut res = 0;
    let mut sum = 0;

    for n in nums {
        if sum >= value {
            break;
        }

        res += 1;
        sum += n;
    }

    res - 1
}
