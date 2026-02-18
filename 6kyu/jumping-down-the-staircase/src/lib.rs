//! <https://www.codewars.com/kata/647d08a2c736e3777c9ae1db/train/rust>

use num_bigint::BigUint;
use unchecked_std::prelude::*;

pub fn get_number_of_ways(steps: usize, max_jump_length: usize) -> BigUint {
    let mut dp = Vec::with_capacity(steps + 1);
    unsafe { dp.push_unchecked(1u8.into()) };

    let mut window_sum = BigUint::from(1u8);

    for step in 1..=steps {
        unsafe { dp.push_unchecked(window_sum.clone()) };
        window_sum <<= 1;
        if step >= max_jump_length {
            window_sum -= unsafe { dp.get_unchecked(step - max_jump_length) };
        }
    }

    dp.pop().unwrap()
}
