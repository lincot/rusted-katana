//! <https://www.codewars.com/kata/555086d53eac039a2a000083/train/rust>

#![no_std]

pub const fn lovefunc(flower1: u16, flower2: u16) -> bool {
    flower1 % 2 != flower2 % 2
}
