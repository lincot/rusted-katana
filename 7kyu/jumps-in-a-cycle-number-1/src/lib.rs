//! <https://www.codewars.com/kata/63f844fee6be1f0017816ff1/train/rust>

use num_integer::gcd;

pub fn get_jumps(cycle: Vec<i32>, k: i32) -> i32 {
    cycle.len() as i32 / gcd(cycle.len() as _, k)
}
