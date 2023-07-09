//! <https://www.codewars.com/kata/52b7ed099cdc285c300001cd/train/rust>

#![no_std]

extern crate alloc;

pub fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    let mut intervals = intervals.to_vec();
    if intervals.len() < 256 {
        intervals.sort_unstable_by_key(|&(a, _)| a);
    } else {
        radsort::sort_by_key(&mut intervals, |&(a, _)| a);
    }

    let mut res = 0;
    let mut end = i32::MIN;
    for (a, b) in intervals {
        if end < a {
            end = a;
        }
        if end < b {
            res += b - end;
            end = b;
        }
    }
    res
}
