//! <https://www.codewars.com/kata/5b45e4b3f41dd36bf9000090/train/rust>

use core::hint::unreachable_unchecked;
use unchecked_std::prelude::*;

pub fn sequence(x: u8) -> Vec<u8> {
    // we assume that `x` is never `> 100`,
    // which seems to hold true for all translations

    let mut res = Vec::with_capacity(100);

    if x < 10 {
        for i in 1..=x {
            unsafe { res.push_unchecked(i) };
        }
        return res;
    }

    for d in 1..10 {
        unsafe { res.push_unchecked(d) };
        if x >= 10 * d {
            for n in 10 * d..(10 * d + 10).min(x + 1) {
                unsafe { res.push_unchecked(n) };
            }
        }
    }

    if x >= 100 {
        if res.len() == res.capacity() {
            unsafe { unreachable_unchecked() };
        }
        res.insert(2, 100);
    }

    res
}
