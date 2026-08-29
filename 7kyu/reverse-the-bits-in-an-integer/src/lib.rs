//! <https://www.codewars.com/kata/5959ec605595565f5c00002b/train/rust>

pub const fn reverse_bits(n: u32) -> u32 {
    n.reverse_bits().wrapping_shr(n.leading_zeros())
}
