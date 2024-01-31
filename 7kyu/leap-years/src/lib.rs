//! <https://www.codewars.com/kata/526c7363236867513f0005ca/train/rust>

pub const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
