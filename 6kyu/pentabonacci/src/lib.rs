//! <https://www.codewars.com/kata/55c9172ee4bb15af9000005d/train/rust>

pub const fn count_odd_pentafib(n: u16) -> u16 {
    if n == 0 {
        0
    } else {
        n.saturating_sub(1) / 6 + n.saturating_sub(2) / 6 + 1
    }
}
