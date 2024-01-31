//! <https://www.codewars.com/kata/5a91a7c5fd8c061367000002/train/rust>

use vqsort::VqSort;

pub fn minimum_steps(nums: &[i32], value: i32) -> usize {
    let mut nums = nums.to_vec();
    VqSort::sort_ascending(&mut nums);

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
