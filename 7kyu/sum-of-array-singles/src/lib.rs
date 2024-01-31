//! <https://www.codewars.com/kata/59f11118a5e129e591000134/train/rust>

use core::ops::BitXor;

pub fn repeats(arr: &[i32]) -> i32 {
    let xor = arr.iter().fold(0, BitXor::bitxor);
    let bit = 1 << xor.trailing_zeros();
    let first = arr.iter().filter(|&x| x & bit != 0).fold(0, BitXor::bitxor);
    first + (xor ^ first)
}
