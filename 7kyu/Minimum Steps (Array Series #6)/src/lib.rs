//! <https://www.codewars.com/kata/5a91a7c5fd8c061367000002/train/rust>

#![feature(binary_heap_into_iter_sorted)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn minimum_steps(nums: &[i32], value: i32) -> usize {
    // arbitrary border
    if (value as usize) < *nums.first().unwrap() as usize * nums.len() / 8 {
        minimum_steps_heap(nums, value)
    } else {
        minimum_steps_sort(nums, value)
    }
}

fn minimum_steps_heap(nums: &[i32], value: i32) -> usize {
    let mut sum = 0;
    BinaryHeap::from_iter(nums.iter().map(Reverse))
        .into_iter_sorted()
        .take_while(|&Reverse(n)| {
            sum += n;
            sum < value
        })
        .count()
}

fn minimum_steps_sort(nums: &[i32], value: i32) -> usize {
    let mut nums = nums.to_vec();
    nums.sort_unstable();

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
