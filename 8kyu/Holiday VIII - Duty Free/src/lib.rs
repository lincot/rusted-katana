//! <https://www.codewars.com/kata/57e92e91b63b6cbac20001e5/train/rust>

pub fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
    ((holiday_cost as f64 * 100.) / (discount as f64 * price as f64)) as _
}
