//! <https://www.codewars.com/kata/64ef24b0679cdc004d08169e/train/rust>

#![no_std]

pub fn min_turns(current: &str, target: &str) -> u8 {
    assert!(current.len() == 4 && target.len() == 4);
    current
        .bytes()
        .zip(target.bytes())
        .map(|(a, b)| {
            let diff = a.abs_diff(b);
            diff.min(10 - diff)
        })
        .sum()
}
