//! <https://www.codewars.com/kata/5a512f6a80eba857280000fc/train/rust>

pub fn nth_smallest(nums: &[i32], pos: usize) -> i32 {
    let mut nums = nums.to_vec();
    *nums.select_nth_unstable(pos - 1).1
}
