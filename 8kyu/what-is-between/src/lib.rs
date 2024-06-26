//! <https://www.codewars.com/kata/55ecd718f46fba02e5000029/train/rust>

use unchecked_std::prelude::*;

pub fn between(a: i16, b: i16) -> Vec<i16> {
    if a >= b {
        return vec![];
    }
    let mut res = Vec::with_capacity((b as i32 - a as i32) as usize + 1);
    let mut i = a;
    loop {
        unsafe { res.push_unchecked(i) };
        if i == b {
            break;
        }
        i += 1;
    }
    res
}
