//! <https://www.codewars.com/kata/56d8f14cba01a83cdb0002a2/train/rust>

#![no_std]

pub const fn get_positions(step: u32) -> [u32; 3] {
    [step % 3, step / 3 % 3, step / 9 % 3]
}
