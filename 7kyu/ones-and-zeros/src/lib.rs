//! <https://www.codewars.com/kata/578553c3a1b8d5c40300037c/train/rust>

pub fn binary_slice_to_number(slice: &[u32]) -> u32 {
    slice.iter().fold(0, |acc, b| (acc << 1) + b)
}
