//! <https://www.codewars.com/kata/5963c18ecb97be020b0000a2/train/rust>

use digital::prelude::*;
use unchecked_std::prelude::*;

pub fn derive(coefficient: u32, exponent: u32) -> String {
    let mut res = String::with_capacity(2 * u32::MAX_LEN_BASE10 + 2);
    unsafe {
        res.write_int_unchecked(coefficient * exponent);
        res.push_str_unchecked("x^");
        res.write_int_unchecked(exponent - 1);
    }
    res
}
