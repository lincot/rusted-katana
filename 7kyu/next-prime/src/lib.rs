//! <https://www.codewars.com/kata/58e230e5e24dde0996000070/train/rust>

pub fn next_prime(n: u64) -> u64 {
    num_prime::nt_funcs::next_prime(&n, None).unwrap()
}
