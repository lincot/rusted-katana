//! <https://www.codewars.com/kata/5a4e3782880385ba68000018/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use digital::WriteNumUnchecked;

pub fn balanced_num(n: u64) -> String {
    fn to_digits(n: u64) -> heapless::Vec<u8, 20> {
        let mut digits = heapless::Vec::new();
        unsafe { digits.write_num_unchecked(n, 10, false, false) };
        digits
    }

    let digits = to_digits(n);
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
