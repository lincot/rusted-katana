//! <https://www.codewars.com/kata/5a5f3034cadebf76db000023/train/rust>

pub fn show_bits(n: i32) -> [u8; 32] {
    let mut res = [0; i32::BITS as _];
    for i in 0..i32::BITS {
        res[(i32::BITS - 1 - i) as usize] = (n & 1 << i != 0) as _;
    }
    res
}
