//! <https://www.codewars.com/kata/62f17be8356b63006a9899dc/train/rust>

pub const fn trilingual_democracy(group: &[u8; 3]) -> u8 {
    group[0] ^ group[1] ^ group[2]
}
