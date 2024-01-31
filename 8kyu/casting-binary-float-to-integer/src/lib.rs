//! <https://www.codewars.com/kata/5f1804560873b20023e8244a/train/rust>

pub fn convert_to_i32(f: f32) -> i32 {
    f.to_bits() as _
}
