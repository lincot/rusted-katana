//! <https://www.codewars.com/kata/5808e2006b65bff35500008f/train/rust>

use unchecked_std::prelude::*;

pub fn position(c: char) -> String {
    let mut res = String::with_capacity("Position of alphabet: 26".len());
    unsafe {
        res.push_str_unchecked("Position of alphabet: ");
        let num = (c as u8) - b'a' + 1;
        if num > 9 {
            if num <= 19 {
                res.as_mut_vec().push_unchecked(b'1');
                res.as_mut_vec().push_unchecked(b'0' + num - 10);
            } else {
                res.as_mut_vec().push_unchecked(b'2');
                res.as_mut_vec().push_unchecked(b'0' + num - 20);
            }
        } else {
            res.as_mut_vec().push_unchecked(b'0' + num);
        }
    }
    res
}
