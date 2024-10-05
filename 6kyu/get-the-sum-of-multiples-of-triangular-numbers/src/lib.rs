//! <https://www.codewars.com/kata/566afbfc8595f2e751000040/train/rust>

use num_bigint::BigUint;

pub fn sum_mult_triangnum(n: u32, m: u32) -> BigUint {
    let mut lcm = 1u8.into();
    for x in 2..n + 1 {
        lcm = lcm_big_small(lcm, triangle(x));
    }
    lcm * triangle(m)
}

const fn triangle(x: u32) -> u32 {
    x * (x + 1) / 2
}

fn lcm_big_small(a: BigUint, b: u32) -> BigUint {
    let g = gcd_big_small(&a, b);
    a * b / g
}

fn gcd_big_small(a: &BigUint, b: u32) -> BigUint {
    gcd(b, unsafe { (a % b).try_into().unwrap_unchecked() }).into()
}

const fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
