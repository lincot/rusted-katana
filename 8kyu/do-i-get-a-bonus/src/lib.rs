//! <https://www.codewars.com/kata/56f6ad906b88de513f000d96/train/rust>

use digital::prelude::*;
use unchecked_std::prelude::*;

pub fn bonus_time(mut salary: u64, bonus: bool) -> String {
    if bonus {
        salary *= 10;
    }

    let mut res = String::with_capacity("¥".len() + 20);
    unsafe {
        res.push_str_unchecked("¥");
        res.write_int_unchecked(salary);
    }
    res
}
