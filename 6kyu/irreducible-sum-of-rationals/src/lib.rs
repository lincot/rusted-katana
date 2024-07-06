//! <https://www.codewars.com/kata/5517fcb0236c8826940003c9/train/rust>

use num_integer::{gcd, lcm};

pub fn sum_fracts(l: Vec<(i64, i64)>) -> Option<(i64, i64)> {
    let first_denom = l.first().map(|x| x.1)?;
    let lcm_ = l[1..]
        .iter()
        .fold(first_denom, |acc, &(_, denom)| lcm(acc, denom));
    let sum = l.iter().map(|(numer, denom)| numer * lcm_ / denom).sum();
    let gcd2 = gcd(sum, lcm_);
    Some((sum / gcd2, lcm_ / gcd2))
}
