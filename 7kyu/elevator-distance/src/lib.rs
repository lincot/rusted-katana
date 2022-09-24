//! <https://www.codewars.com/kata/59f061773e532d0c87000d16/train/rust>

#![no_std]

pub fn elevator_distance(floors: &[i16]) -> i16 {
    floors.windows(2).map(|s| (s[0] - s[1]).abs()).sum()
}
