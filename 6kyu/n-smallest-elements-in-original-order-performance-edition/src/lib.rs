//! <https://www.codewars.com/kata/5aeed69804a92621a7000077/train/rust>

use unchecked_std::prelude::*;

pub fn performant_smallest(arr: &[u32], n: usize) -> Vec<u32> {
    let mut counts = [0; 51];
    for &key in arr {
        counts[key as usize] += 1;
    }

    let mut allowance = n;
    let mut max_fully_allowed = 0;
    let mut partially_allowed = 0;
    for (key, &count) in counts.iter().enumerate() {
        if count <= allowance {
            max_fully_allowed = key as _;
            allowance -= count;
        } else {
            partially_allowed = key as _;
            break;
        }
    }

    let mut res = Vec::with_capacity(n);
    for &key in arr {
        if res.len() == n {
            break;
        }
        if key <= max_fully_allowed {
            unsafe { res.push_unchecked(key) };
        } else if key == partially_allowed && allowance > 0 {
            unsafe { res.push_unchecked(key) };
            allowance -= 1;
        }
    }
    res
}
