//! <https://www.codewars.com/kata/58845a92bd573378f4000035/train/rust>

pub const fn swap_adjacent_bits(n: u32) -> u32 {
    (n & 0xaaaa_aaaa) >> 1 | (n & 0x5555_5555) << 1
}
