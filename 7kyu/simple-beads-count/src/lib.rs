//! <https://www.codewars.com/kata/58712dfa5c538b6fc7000569/train/rust>

#![no_std]

pub const fn count_red_beads(n: u32) -> u32 {
    (n * 2).saturating_sub(2)
}
