//! <https://www.codewars.com/kata/55f8a9c06c018a0d6e000132/train/rust>

pub fn validate_pin(pin: &str) -> bool {
    ([4, 6].contains(&pin.len())) && pin.as_bytes().iter().all(u8::is_ascii_digit)
}
