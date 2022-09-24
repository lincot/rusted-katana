//! <https://www.codewars.com/kata/59ca8246d751df55cc00014c/train/rust>

#![no_std]

pub const fn hero(bullets: u16, dragons: u16) -> bool {
    bullets >= dragons * 2
}
