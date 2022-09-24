//! <https://www.codewars.com/kata/5a61a846cadebf9738000076/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn peak(arr: &[u32]) -> Option<usize> {
    let mut sums = Vec::with_capacity(arr.len());
    unsafe { sums.set_len(arr.len()) };
    let mut sums = sums.into_boxed_slice();
    let mut sums_ptr = sums.as_mut_ptr();
    let mut sum = 0;

    for n in arr {
        unsafe {
            *sums_ptr = sum;
            sums_ptr = sums_ptr.add(1);
        }
        sum += n;
    }

    if sums.len() != arr.len() {
        unsafe { core::hint::unreachable_unchecked() };
    }

    (0..arr.len()).find(|&i| sums[i] == sum - arr[i] - sums[i])
}
