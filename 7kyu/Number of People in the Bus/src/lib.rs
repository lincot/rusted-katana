//! <https://www.codewars.com/kata/5648b12ce68d9daa6b000099/train/rust>

pub fn number(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops.iter().fold(0, |acc, (on, off)| acc + on - off)
}
