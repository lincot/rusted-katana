//! <https://www.codewars.com/kata/696eacb39271f8aa43b61841/train/rust>

pub const fn toggle_bit(n: u32, pos: u32) -> u32 {
    n ^ (1 << pos)
}

pub const fn set_bit(n: u32, pos: u32) -> u32 {
    n | (1 << pos)
}

pub const fn clear_bit(n: u32, pos: u32) -> u32 {
    n & !(1 << pos)
}

pub const fn is_bit_set(n: u32, pos: u32) -> bool {
    (n >> pos) & 1 == 1
}

pub const fn set_multiple_bits(n: u32, mask: u32) -> u32 {
    n | mask
}

pub const fn clear_multiple_bits(n: u32, mask: u32) -> u32 {
    n & !mask
}

pub const fn toggle_multiple_bits(n: u32, mask: u32) -> u32 {
    n ^ mask
}
