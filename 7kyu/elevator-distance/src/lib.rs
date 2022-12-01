//! <https://www.codewars.com/kata/59f061773e532d0c87000d16/train/rust>

#![no_std]
#![feature(array_windows)]

pub fn elevator_distance(floors: &[i16]) -> i16 {
    floors.array_windows().map(|[a, b]| (a - b).abs()).sum()
}
