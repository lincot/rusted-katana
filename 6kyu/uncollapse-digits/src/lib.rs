//! <https://www.codewars.com/kata/5a626fc7fd56cb63c300008c/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use unchecked::{ExtendFromSliceUnchecked, PushUnchecked};

pub fn uncollapse(digits: &str) -> String {
    if digits.len() < "oneone".len() {
        return digits.into();
    }
    let digits = digits.as_bytes();
    let mut res = Vec::with_capacity(digits.len() + digits.len() / 3);
    unsafe { res.extend_from_slice_unchecked(&digits[..3]) };
    for trio in digits[3..].windows(3) {
        if ([
            b"zer", b"one", b"two", b"thr", b"fou", b"fiv", b"six", b"sev", b"eig", b"nin",
        ])
        .contains(&trio.try_into().unwrap())
        {
            unsafe { res.push_unchecked(b' ') };
        }
        unsafe { res.push_unchecked(trio[0]) };
    }
    unsafe {
        res.extend_from_slice_unchecked(digits.get_unchecked(digits.len() - 2..digits.len()));
        String::from_utf8_unchecked(res)
    }
}
