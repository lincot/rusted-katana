//! <https://www.codewars.com/kata/560b8d7106ede725dd0000e2/train/rust>

pub fn prime_bef_aft(n: u32) -> (u32, u32) {
    (
        num_prime::nt_funcs::prev_prime(&n, None).unwrap(),
        num_prime::nt_funcs::next_prime(&n, None).unwrap(),
    )
}
