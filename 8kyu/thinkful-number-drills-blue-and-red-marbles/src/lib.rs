//! <https://www.codewars.com/kata/5862f663b4e9d6f12b00003b/train/rust>

#![no_std]

pub fn guess_blue(blue_start: u32, red_start: u32, blue_pulled: u32, red_pulled: u32) -> f32 {
    let blue = blue_start - blue_pulled;
    let red = red_start - red_pulled;
    let total = blue + red;

    blue as f32 / total as f32
}
