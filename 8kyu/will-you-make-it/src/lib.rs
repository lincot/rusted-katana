//! <https://www.codewars.com/kata/5861d28f124b35723e00005e/train/rust>

#![no_std]

pub const fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    mpg * gallons >= distance_to_pump
}
