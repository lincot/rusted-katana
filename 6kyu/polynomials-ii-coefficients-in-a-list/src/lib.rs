//! <https://www.codewars.com/kata/5694b4f9a01ae685c400002f/train/rust>

use core::cmp::Ordering;
use digital::prelude::*;
use unchecked_std::prelude::*;

pub fn calc_poly(pol_list: &[i32], x: i32) -> String {
    assert!(pol_list.len() <= MAX_POL_LIST_LEN);
    let mut res = String::with_capacity(get_capacity(pol_list.len()));
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
                res.write_int_unchecked(coef);
                if exp != 0 {
                    res.push_unchecked('*');
                }
            }
            if exp != 0 {
                res.push_unchecked('x');
            }
            if exp > 1 {
                res.push_unchecked('^');
                res.write_int_unchecked(exp);
            }

            is_first = false;
        }

        res.push_str_unchecked(" with x = ");
        res.write_int_unchecked(x);
        res.push_str_unchecked(" the value is ");
        res.write_int_unchecked(value);
    }

    res
}

#[cfg(any(target_pointer_width = "64", test))]
const MAX_POL_LIST_LEN_64: usize = 341_606_371_735_362_065;
#[cfg(any(target_pointer_width = "32", test))]
const MAX_POL_LIST_LEN_32: usize = 79_536_429;
#[cfg(any(target_pointer_width = "16", test))]
const MAX_POL_LIST_LEN_16: usize = 1211;

#[cfg(target_pointer_width = "64")]
const MAX_POL_LIST_LEN: usize = MAX_POL_LIST_LEN_64;
#[cfg(target_pointer_width = "32")]
const MAX_POL_LIST_LEN: usize = MAX_POL_LIST_LEN_32;
#[cfg(target_pointer_width = "16")]
const MAX_POL_LIST_LEN: usize = MAX_POL_LIST_LEN_16;

const fn get_capacity(pol_list_len: usize) -> usize {
    "For  with  the value is ".len()
        + 2 * i32::MAX_LEN_BASE10
        + (i32::MAX_LEN_BASE10 + u32::MAX_LEN_BASE10 + "*x^ + ".len()) * pol_list_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_pol_list_len() {
        assert!(isize::try_from(get_capacity(MAX_POL_LIST_LEN)).is_ok());
        assert!(isize::try_from(get_capacity(MAX_POL_LIST_LEN + 1)).is_err());

        assert!(i64::try_from(get_capacity(MAX_POL_LIST_LEN_64)).is_ok());
        assert!(i64::try_from(get_capacity(MAX_POL_LIST_LEN_64 + 1)).is_err());

        assert!(i32::try_from(get_capacity(MAX_POL_LIST_LEN_32)).is_ok());
        assert!(i32::try_from(get_capacity(MAX_POL_LIST_LEN_32 + 1)).is_err());

        assert!(i16::try_from(get_capacity(MAX_POL_LIST_LEN_16)).is_ok());
        assert!(i16::try_from(get_capacity(MAX_POL_LIST_LEN_16 + 1)).is_err());
    }
}
