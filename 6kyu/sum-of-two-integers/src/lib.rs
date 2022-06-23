//! <https://www.codewars.com/kata/5a9c35e9ba1bb5c54a0001ac/train/rust>

pub fn add(a: i32, b: i32) -> i32 {
    (0..a as u32).chain(0..b as u32).size_hint().0 as i32
}
