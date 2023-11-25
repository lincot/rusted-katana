//! <https://www.codewars.com/kata/6501aa820038a6b0bd098afb/train/rust>

#![no_std]

pub const fn safecracker(start: u8, incs: &(u16, u16, u16)) -> (u8, u8, u8) {
    let (s, a, b, c) = (start as u32, incs.0 as u32, incs.1 as u32, incs.2 as u32);
    (
        ((1_000_000 + s - a) % 100) as u8,
        ((1_000_000 + s - a + b) % 100) as u8,
        ((1_000_000 + s - a + b - c) % 100) as u8,
    )
}
