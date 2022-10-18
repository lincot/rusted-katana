//! <https://www.codewars.com/kata/6167e70fc9bd9b00565ffa4e/train/rust>

#![no_std]

extern crate alloc;
use alloc::boxed::Box;

pub fn barista(coffees: &[u8]) -> u16 {
    let mut coffees: Box<[_]> = coffees.into();
    coffees.sort_unstable();
    let mut sum_of_prev = 0;
    let mut res = coffees.len() as u16 * coffees.len() as u16 - coffees.len() as u16;
    for &coffee in coffees.iter() {
        sum_of_prev += coffee as u16;
        res += sum_of_prev;
    }
    res
}
