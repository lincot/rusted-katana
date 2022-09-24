//! <https://www.codewars.com/kata/5aa1bcda373c2eb596000112/train/rust>

#![no_std]

pub fn max_tri_sum(xs: &[i32]) -> i32 {
    let mut a = i32::MIN;
    let mut b = i32::MIN;
    let mut c = i32::MIN;

    for &x in xs {
        if x > a {
            c = b;
            b = a;
            a = x;
        } else if x > b && x != a {
            c = b;
            b = x;
        } else if x > c && x != a && x != b {
            c = x;
        }
    }

    a + b + c
}
