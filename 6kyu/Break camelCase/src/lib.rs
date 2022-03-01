//! <https://www.codewars.com/kata/5208f99aee097e6552000148/train/rust>

pub fn solution(s: &str) -> String {
    // worst case capacity
    let cap = 2 * s.len();
    let mut res = String::with_capacity(cap);

    for c in s.chars() {
        if c.is_uppercase() {
            res.push(' ');
        }
        res.push(c);
    }

    res
}
