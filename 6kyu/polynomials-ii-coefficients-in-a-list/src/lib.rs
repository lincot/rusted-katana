//! <https://www.codewars.com/kata/5694b4f9a01ae685c400002f/train/rust>

use core::cmp::Ordering;
use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked_core::{PushStrUnchecked, PushUnchecked};

pub fn calc_poly(pol_list: &[i32], x: i32) -> String {
    let mut res = String::with_capacity(
        "For  with  the value is ".len()
            + 2 * i32::MAX_LEN_BASE10
            + (i32::MAX_LEN_BASE10 + u32::MAX_LEN_BASE10 + "*x^ + ".len()) * pol_list.len(),
    );
    let mut value = 0;

    unsafe {
        res.push_str_unchecked("For");

        let mut is_first = true;
        for i in 0..pol_list.len() {
            let mut coef = pol_list[i];

            if coef == 0 {
                continue;
            }

            let exp = (pol_list.len() - 1 - i) as _;
            value += coef * x.pow(exp);

            res.push_unchecked(' ');
            match coef.cmp(&0) {
                Ordering::Less => {
                    res.push_unchecked('-');
                    coef = -coef;
                }
                Ordering::Greater => {
                    if !is_first {
                        res.push_unchecked('+');
                    }
                }
                Ordering::Equal => continue,
            }
            if !is_first {
                res.push_unchecked(' ');
            }
            if coef != 1 || exp == 0 {
                res.write_num_unchecked(coef, 10, false, false);
                if exp != 0 {
                    res.push_unchecked('*');
                }
            }
            if exp != 0 {
                res.push_unchecked('x');
            }
            if exp > 1 {
                res.push_unchecked('^');
                res.write_num_unchecked(exp, 10, false, false);
            }

            is_first = false;
        }

        res.push_str_unchecked(" with x = ");
        res.write_num_unchecked(x, 10, false, false);
        res.push_str_unchecked(" the value is ");
        res.write_num_unchecked(value, 10, false, false);
    }

    res
}
