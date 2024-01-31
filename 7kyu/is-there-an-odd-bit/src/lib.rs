//! <https://www.codewars.com/kata/5d6f49d85e45290016bf4718/train/rust>

pub const fn any_odd(x: u32) -> u8 {
    (x & 0b1010_1010_1010_1010_1010_1010_1010_1010 != 0) as _
}
