//! <https://www.codewars.com/kata/5d65fbdfb96e1800282b5ee0/train/rust>

pub fn wrap(height: u32, width: u32, length: u32) -> u32 {
    2 * (height + width + length + height.min(width).min(length)) + 20
}
