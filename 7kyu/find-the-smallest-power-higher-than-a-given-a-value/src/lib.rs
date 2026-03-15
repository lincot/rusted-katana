//! <https://www.codewars.com/kata/56ba65c6a15703ac7e002075/train/rust>

use num_integer::Roots;

pub fn find_next_power(val: u64, pow_: u32) -> u64 {
    (val.nth_root(pow_) + 1).pow(pow_)
}
