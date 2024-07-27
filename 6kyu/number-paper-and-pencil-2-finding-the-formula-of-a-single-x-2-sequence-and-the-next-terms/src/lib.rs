//! <https://www.codewars.com/kata/63bd8cc3a78e0578b608ac80/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn quadratic_formula(y1: i32, y2: i32, _y3: i32) -> (String, i32, i32) {
    let b = y2 - y1 - 3;
    let c = y1 - b - 1;

    let mut s = String::with_capacity("x^2+x+".len() + 2 * u32::MAX_LEN_BASE10);
    unsafe {
        s.push_str_unchecked("x^2");
        if b != 0 {
            if b > 0 {
                s.push_unchecked('+');
            } else {
                s.push_unchecked('-');
            }
            let b = b.unsigned_abs();
            if b != 1 {
                s.write_num_unchecked(b, 10, false, false);
            }
            s.push_unchecked('x');
        }
        if c != 0 {
            if c > 0 {
                s.push_unchecked('+');
            }
            s.write_num_unchecked(c, 10, false, false);
        }
    }

    let next1 = 16 + 4 * b + c;
    let next2 = 25 + 5 * b + c;

    (s, next1, next2)
}
