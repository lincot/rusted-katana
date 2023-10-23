//! <https://www.codewars.com/kata/5842df8ccbd22792a4000245/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use digital::NumToString;
use unchecked::{ExtendFromSliceUnchecked, PushUnchecked};

pub fn expanded_form(n: u64) -> String {
    if n == 0 {
        return "0".into();
    }
    let digits = n.to_heapless_string(false, false).into_bytes();
    let mut res = Vec::with_capacity((digits.len() * (digits.len() + 7) / 2 - 3) as _);
    unsafe {
        res.push_unchecked(*digits.get_unchecked(0));
        for _ in 0..digits.len() - 1 {
            res.push_unchecked(b'0');
        }
        for (i, &d) in (0..digits.len() - 1)
            .rev()
            .zip(&digits[1..])
            .filter(|(_, &b)| b != b'0')
        {
            res.extend_from_slice_unchecked(b" + ");
            res.push_unchecked(d);
            for _ in 0..i {
                res.push_unchecked(b'0');
            }
        }
        String::from_utf8_unchecked(res)
    }
}
