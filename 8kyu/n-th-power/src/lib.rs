//! <https://www.codewars.com/kata/57d814e4950d8489720008db/train/rust>

pub fn index(nums: &[u64], n: usize) -> Option<u64> {
    nums.get(n).map(|x| x.pow(n as _))
}
