//! <https://www.codewars.com/kata/59f44c7bd4b36946fd000052/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked::{ExtendFromSliceUnchecked, PushUnchecked};

pub fn hist(s: &str) -> String {
    let (mut u, mut w, mut x, mut z) = (0, 0, 0, 0);
    for &b in s.as_bytes() {
        match b {
            b'u' => u += 1,
            b'w' => w += 1,
            b'x' => x += 1,
            b'z' => z += 1,
            _ => {}
        }
    }
    let mut res = Vec::with_capacity(u + w + x + z + 4 * (1 + 2 + usize::MAX_LEN_BASE10 + 5 + 1));
    for (symbol, amount) in [(b'u', u), (b'w', w), (b'x', x), (b'z', z)] {
        if amount == 0 {
            continue;
        }
        unsafe {
            res.push_unchecked(symbol);
            res.extend_from_slice_unchecked(b"  ");
            res.write_num_unchecked(amount, 10, false, false);
            res.extend_from_slice_unchecked(b"     ");
            for _ in 0..amount {
                res.push_unchecked(b'*');
            }
            res.push_unchecked(b'\r');
        }
    }
    res.pop();
    unsafe { String::from_utf8_unchecked(res) }
}
