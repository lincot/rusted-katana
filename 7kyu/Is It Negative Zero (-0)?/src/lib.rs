//! <https://www.codewars.com/kata/5c5086287bc6600001c7589a/train/rust>

pub fn is_negative_zero(n: f32) -> bool {
    n.to_bits() == 0b10000000000000000000000000000000
}
