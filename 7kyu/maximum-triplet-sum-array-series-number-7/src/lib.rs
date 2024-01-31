//! <https://www.codewars.com/kata/5aa1bcda373c2eb596000112/train/rust>

pub fn max_tri_sum(xs: &[i32]) -> i32 {
    let mut a = i32::MIN;
    let mut b = i32::MIN;
    let mut c = i32::MIN;
    for &x in xs {
        if x > a {
            (a, b, c) = (x, a, b);
        } else if x > b && x != a {
            (b, c) = (x, b);
        } else if x > c && x != a && x != b {
            c = x;
        }
    }
    a + b + c
}
