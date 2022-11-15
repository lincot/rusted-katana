//! <https://www.codewars.com/kata/5913152be0b295cf99000001/train/rust>

#![no_std]

pub const fn divisions(n: u32, divisor: u32) -> u32 {
    n.ilog(divisor)
}
