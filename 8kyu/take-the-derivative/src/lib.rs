//! <https://www.codewars.com/kata/5963c18ecb97be020b0000a2/train/rust>

use lexical_core::write_unchecked;
use my_prelude::prelude::*;

pub fn derive(coefficient: u32, exponent: u32) -> String {
    let mut res = String::with_capacity(10 + 2 + 10);
    unsafe {
        let len1 = write_unchecked(
            coefficient * exponent,
            core::slice::from_raw_parts_mut(res.as_mut_ptr(), res.capacity()),
        )
        .len();
        res.as_mut_vec().set_len(len1);
        res.push_str_unchecked("x^");
        let len2 = write_unchecked(
            exponent - 1,
            core::slice::from_raw_parts_mut(
                res.as_mut_ptr().add(len1 + 2),
                res.capacity() - (len1 + 2),
            ),
        )
        .len();
        res.as_mut_vec().set_len(len1 + 2 + len2);
    }
    res
}
