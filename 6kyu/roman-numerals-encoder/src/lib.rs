//! <https://www.codewars.com/kata/51b62bf6a9c58071c600001b/train/rust>

use unchecked_std::prelude::*;

pub fn num_as_roman(mut num: i32) -> String {
    assert!((1..=3999).contains(&num));
    let mut res = String::with_capacity("MMMDCCCLXXXVIII".len());
    unsafe {
        while num >= 1000 {
            res.push_unchecked('M');
            num -= 1000;
        }
        if num >= 900 {
            res.push_str_unchecked("CM");
            num -= 900;
        }
        if num >= 500 {
            res.push_unchecked('D');
            num -= 500;
        }
        if num >= 400 {
            res.push_str_unchecked("CD");
            num -= 400;
        }
        while num >= 100 {
            res.push_unchecked('C');
            num -= 100;
        }
        if num >= 90 {
            res.push_str_unchecked("XC");
            num -= 90;
        } else if num >= 50 {
            res.push_str_unchecked("L");
            num -= 50;
        }
        if num >= 40 {
            res.push_str_unchecked("XL");
            num -= 40;
        }
        while num >= 10 {
            res.push_unchecked('X');
            num -= 10;
        }
        if num >= 9 {
            res.push_str_unchecked("IX");
            return res;
        }
        if num >= 5 {
            res.push_unchecked('V');
            num -= 5;
        }
        if num >= 4 {
            res.push_str_unchecked("IV");
            return res;
        }
        while num != 0 {
            res.push_unchecked('I');
            num -= 1;
        }
    }
    res
}
