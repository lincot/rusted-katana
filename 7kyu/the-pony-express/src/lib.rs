//! <https://www.codewars.com/kata/5b18e9e06aefb52e1d0001e9/train/rust>

#![no_std]

pub fn riders(stations: &[u32]) -> u32 {
    stations
        .iter()
        .fold((0, 1), |(covered_by_rider, riders), &distance| {
            if covered_by_rider + distance > 100 {
                (distance, riders + 1)
            } else {
                (covered_by_rider + distance, riders)
            }
        })
        .1
}
