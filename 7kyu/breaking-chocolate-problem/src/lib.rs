//! <https://www.codewars.com/kata/534ea96ebb17181947000ada/train/rust>

pub const fn break_chocolate(n: u32, m: u32) -> u64 {
    (n as u64 * m as u64).saturating_sub(1)
}
