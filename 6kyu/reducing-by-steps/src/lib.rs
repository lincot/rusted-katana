//! <https://www.codewars.com/kata/56efab15740d301ab40002ee/train/rust>

use num_integer::{gcd, lcm};

pub const fn som(x: i64, y: i64) -> i64 {
    x + y
}

pub fn maxi(x: i64, y: i64) -> i64 {
    x.max(y)
}

pub fn mini(x: i64, y: i64) -> i64 {
    x.min(y)
}

pub fn gcdi(m: i64, n: i64) -> i64 {
    gcd(m.abs(), n.abs())
}

pub fn lcmu(a: i64, b: i64) -> i64 {
    lcm(a.abs(), b.abs())
}

pub fn oper_array(f: impl Fn(i64, i64) -> i64, a: &[i64], init: i64) -> Vec<i64> {
    let mut prev = init;
    a.iter()
        .map(|&x| {
            let r = f(prev, x);
            prev = r;
            r
        })
        .collect()
}
