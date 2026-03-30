//! <https://www.codewars.com/kata/65f361be2b30ec19b78d758f/train/rust>

use digital::prelude::*;
use unchecked_std::prelude::*;

pub fn frog_contest(n: u32) -> String {
    let sum = |x| x * (x + 1) / 2;

    let chris = sum(n);
    let tom = sum(chris / 2);
    let cat = sum(chris + tom);

    let mut res = String::with_capacity(
        "Chris ate  flies, Tom ate  flies and Cat ate  flies".len() + 3 * u32::MAX_LEN_BASE10,
    );
    unsafe {
        res.push_str_unchecked("Chris ate ");
        res.write_int_unchecked(chris);
        res.push_str_unchecked(" flies, Tom ate ");
        res.write_int_unchecked(tom);
        res.push_str_unchecked(" flies and Cat ate ");
        res.write_int_unchecked(cat);
        res.push_str_unchecked(" flies");
    }
    res
}
