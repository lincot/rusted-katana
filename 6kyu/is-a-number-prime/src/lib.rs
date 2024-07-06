//! <https://www.codewars.com/kata/5262119038c0985a5b00029f/train/rust>

use num_prime::nt_funcs::is_prime64;

pub fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false;
    }
    let x = x as u64;
    is_prime64(x)
}
