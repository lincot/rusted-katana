//! <https://www.codewars.com/kata/57e92e91b63b6cbac20001e5/train/rust>

#![no_std]

pub fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
    let x = discount * price;
    assert!(x > 0);
    100 * holiday_cost / x
}
