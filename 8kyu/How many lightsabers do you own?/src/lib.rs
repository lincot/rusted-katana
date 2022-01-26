//! <https://www.codewars.com/kata/51f9d93b4095e0a7200001b8/train/rust>

pub const fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
    match name.as_bytes() {
        b"Zach" => 18,
        _ => 0,
    }
}
