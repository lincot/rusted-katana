//! <https://www.codewars.com/kata/538948d4daea7dc4d200023f/train/rust>

pub const fn convert_bits(a: u32, b: u32) -> u32 {
    (a ^ b).count_ones()
}
