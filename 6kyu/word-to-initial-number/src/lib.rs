//! <https://www.codewars.com/kata/5bb148b840196d1be50000b1/train/rust>

#![allow(internal_features)]
#![feature(unicode_internals)]

use core::unicode::conversions::to_lower;

pub fn convert(word: &str) -> u64 {
    let mut res = 0;
    let mut map = heapless::FnvIndexMap::<_, _, 16>::new();
    for c in word.chars().map(to_lower) {
        let len = map.len() as u8;
        let digit = match map.entry(c) {
            heapless::Entry::Occupied(e) => *e.get(),
            heapless::Entry::Vacant(e) => {
                e.insert(len).unwrap();
                len
            }
        };
        let digit = if digit == 0 {
            1
        } else if digit == 1 {
            0
        } else {
            digit
        };
        res *= 10;
        res += digit as u64;
    }
    res
}
