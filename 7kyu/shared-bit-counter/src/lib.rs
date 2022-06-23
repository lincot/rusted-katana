//! <https://www.codewars.com/kata/58a5aeb893b79949eb0000f1/train/rust>

pub const fn shared_bits(a: u32, b: u32) -> bool {
    (a & b).count_ones() > 1
}
