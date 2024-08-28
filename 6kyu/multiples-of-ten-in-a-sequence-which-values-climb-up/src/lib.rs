//! <https://www.codewars.com/kata/561d54055e399e2f62000045/train/rust>

use num_bigint::BigUint;

pub fn find_multiples_of_10_sf(n: u32) -> BigUint {
    let three = BigUint::from(3u8);
    let k = 4 * n - 1;
    (three.pow(k) + three) << (k - 2)
}
