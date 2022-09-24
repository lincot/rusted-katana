//! <https://www.codewars.com/kata/55f8a9c06c018a0d6e000132/train/rust>

#![no_std]

pub fn validate_pin(pin: &str) -> bool {
    ([4, 6].contains(&pin.len())) && pin.bytes().all(|b| (b'0'..=b'9').contains(&b))
}
