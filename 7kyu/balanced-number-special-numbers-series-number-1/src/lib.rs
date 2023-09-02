//! <https://www.codewars.com/kata/5a4e3782880385ba68000018/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use digital::NumToString;

pub fn balanced_num(n: u64) -> String {
    let digits = n.to_heapless_string(false, false).into_bytes();
    if unsafe { digits.get_unchecked(..((digits.len() - 1) / 2)) }
        .iter()
        .sum::<u8>()
        == unsafe { digits.get_unchecked((digits.len() / 2 + 1)..) }
            .iter()
            .sum()
    {
        "Balanced".into()
    } else {
        "Not Balanced".into()
    }
}
