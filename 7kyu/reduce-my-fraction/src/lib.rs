//! <https://www.codewars.com/kata/576400f2f716ca816d001614/train/rust>

use num_integer::gcd;

pub fn reduce_fraction(fraction: (u32, u32)) -> (u32, u32) {
    let gcd_ = gcd(fraction.0, fraction.1);
    (fraction.0 / gcd_, fraction.1 / gcd_)
}
