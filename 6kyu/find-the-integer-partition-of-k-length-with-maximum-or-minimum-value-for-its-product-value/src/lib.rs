//! <https://www.codewars.com/kata/57161cb0b436cf0911000819/train/rust>

use unchecked::PushUnchecked;

pub fn find_spec_partition(n: u32, k: u32, com: &str) -> Vec<u32> {
    if com.as_bytes()[1] == b'a' {
        let mut res = Vec::with_capacity(k as _);
        for _ in 0..n % k {
            unsafe { res.push_unchecked(n / k + 1) };
        }
        for _ in 0..k - n % k {
            unsafe { res.push_unchecked(n / k) };
        }
        res
    } else {
        let mut res = vec![1; k as _];
        res[0] = n + 1 - k;
        res
    }
}
