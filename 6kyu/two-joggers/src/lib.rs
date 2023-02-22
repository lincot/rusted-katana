//! <https://www.codewars.com/kata/5274d9d3ebc3030802000165/train/rust>

#![no_std]

use num_integer::Integer;

pub fn nbr_of_laps(x: u16, y: u16) -> (u16, u16) {
    assert!(x != 0 && y != 0);
    let g = x.gcd(&y);
    (y / g, x / g)
}
