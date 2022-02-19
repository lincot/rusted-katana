//! <https://www.codewars.com/kata/577b9960df78c19bca00007e/train/rust>

pub const fn find_digit(num: i32, nth: i32) -> i32 {
    if nth <= 0 {
        -1
    } else if nth > 9 {
        0
    } else {
        num.abs() / 10i32.pow(nth as u32 - 1) % 10
    }
}
