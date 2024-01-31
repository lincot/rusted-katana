//! <https://www.codewars.com/kata/57a4d500e298a7952100035d/train/rust>

pub fn hex_to_dec(hex_string: &str) -> u32 {
    u32::from_str_radix(hex_string, 16).unwrap()
}
