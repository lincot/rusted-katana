//! <https://www.codewars.com/kata/57a5c31ce298a7e6b7000334/train/rust>

pub fn bin_to_decimal(inp: &str) -> i32 {
    i32::from_str_radix(inp, 2).unwrap()
}
