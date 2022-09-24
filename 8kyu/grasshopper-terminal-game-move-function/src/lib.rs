//! <https://www.codewars.com/kata/563a631f7cbbc236cf0000c2/train/rust>

#![no_std]

pub const fn move_hero(position: u32, roll: u32) -> u32 {
    position + 2 * roll
}
