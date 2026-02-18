//! <https://www.codewars.com/kata/66c9186bb01defccbd40449d/train/rust>

pub fn gould() -> impl Iterator<Item = u8> {
    (0_u32..).map(|i| i.count_ones() as u8)
}
