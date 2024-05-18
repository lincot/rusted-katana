//! <https://www.codewars.com/kata/588e2a1ad1140d31cb00008c/train/rust>

use unchecked_std::prelude::*;

pub fn generate_pairs(m: i16, n: i16) -> Vec<(i16, i16)> {
    let t = if n >= m {
        (n as i32 - m as i32 + 1) as _
    } else {
        0
    };
    let mut res = Vec::with_capacity((t * t + t) / 2);
    if t == 0 {
        return res;
    }
    let mut first = m;
    loop {
        let mut second = first;
        loop {
            unsafe { res.push_unchecked((first, second)) };
            if second == n {
                break;
            }
            second += 1;
        }
        if first == n {
            break;
        }
        first += 1;
    }
    res
}
