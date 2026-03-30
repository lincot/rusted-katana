//! <https://www.codewars.com/kata/56541980fa08ab47a0000040/train/rust>

use digital::prelude::*;
use unchecked_std::prelude::*;

pub fn printer_error(s: &str) -> String {
    let malformed = s.bytes().filter(|&b| b > b'm').count();
    let all = s.len();

    let mut res = String::with_capacity(2 * usize::MAX_LEN_BASE10 + 1);
    unsafe {
        res.write_int_unchecked(malformed);
        res.push_unchecked('/');
        res.write_int_unchecked(all);
    }
    res
}
