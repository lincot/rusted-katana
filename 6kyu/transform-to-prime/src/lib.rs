//! <https://www.codewars.com/kata/5a946d9fba1bb5135100007c/train/rust>

use num_prime::nt_funcs::next_prime;

pub fn minimum_number(xs: &[u32]) -> u32 {
    let sum = xs.iter().sum::<u32>();
    next_prime(&(sum - 1), None).unwrap() - sum
}
