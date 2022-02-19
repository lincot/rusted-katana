//! <https://www.codewars.com/kata/5d6f49d85e45290016bf4718/train/rust>

pub const fn any_odd(x: u32) -> u8 {
    (x & 0b10101010101010101010101010101010 != 0) as _
}
