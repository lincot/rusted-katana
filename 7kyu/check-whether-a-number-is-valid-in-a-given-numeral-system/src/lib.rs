//! <https://www.codewars.com/kata/67757660c552a3a7ef9aaceb/train/rust>

pub fn validate_base(num: &str, base: u32) -> bool {
    let threshold = if base <= 10 {
        b'0' + base as u8
    } else {
        b'A' + base as u8 - 10
    };
    num.bytes().all(|b| b < threshold)
}
