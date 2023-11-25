//! <https://www.codewars.com/kata/56ff1667cc08cacf4b00171b/train/rust>

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use core::cmp::Reverse;

pub fn count_vegetables(s: &str) -> Vec<(u32, String)> {
    let mut vegetables: [(_, String); 10] = [
        (0, "turnip".into()),
        (0, "tofu".into()),
        (0, "potato".into()),
        (0, "pepper".into()),
        (0, "onion".into()),
        (0, "mushroom".into()),
        (0, "cucumber".into()),
        (0, "celery".into()),
        (0, "carrot".into()),
        (0, "cabbage".into()),
    ];
    for word in s.as_bytes().split(|&b| b == b' ') {
        if let Some((count, _)) = vegetables.iter_mut().find(|(_, v)| word == v.as_bytes()) {
            *count += 1;
        }
    }
    vegetables.sort_by_key(|&(c, _)| Reverse(c));
    let first_zero_pos = vegetables
        .iter()
        .rposition(|&(c, _)| c != 0)
        .map_or(vegetables.len(), |x| x + 1);
    vegetables[..first_zero_pos].into()
}
