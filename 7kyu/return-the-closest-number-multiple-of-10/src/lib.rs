//! <https://www.codewars.com/kata/58249d08b81f70a2fc0001a4/train/rust>

pub const fn closest_multiple_of_10(n: u32) -> u32 {
    (n + 5) / 10 * 10
}
