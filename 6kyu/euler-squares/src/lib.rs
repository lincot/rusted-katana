//! <https://www.codewars.com/kata/6470e15f4f0b26052c6151cd/train/rust>

use unchecked_std::prelude::*;

pub fn create_euler_square(n: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    assert!(n < i32::MAX as usize);
    let mut template = Vec::with_capacity(2 * n);
    for x in 1..n as i32 + 1 {
        unsafe { template.push_unchecked(x) };
    }
    unsafe { template.extend_from_within_unchecked(0..n) };
    let a: Vec<_> = (0..n)
        .map(|i| unsafe { template.get_unchecked(i..i + n) }.into())
        .collect();
    let b = a.iter().rev().cloned().collect();
    (a, b)
}
