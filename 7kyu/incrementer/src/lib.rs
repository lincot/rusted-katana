//! <https://www.codewars.com/kata/590e03aef55cab099a0002e8/train/rust>

pub fn incrementer(nums: &[u32]) -> Vec<u32> {
    (1..).zip(nums).map(|(i, x)| (i + x) % 10).collect()
}
