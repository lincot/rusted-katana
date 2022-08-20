//! <https://www.codewars.com/kata/5865cff66b5699883f0001aa/train/rust>

use my_prelude::prelude::*;

pub fn to_time(seconds: u32) -> String {
    let mut minutes = seconds / 60;
    let hours = minutes / 60;
    minutes %= 60;

    let mut res = String::with_capacity(" hour(s) and  minute(s)".len() + 7 + 2);
    unsafe {
        res.write_num_unchecked(hours);
        res.push_str_unchecked(" hour(s) and ");
        res.write_num_unchecked(minutes);
        res.push_str_unchecked(" minute(s)");
    }
    res
}
