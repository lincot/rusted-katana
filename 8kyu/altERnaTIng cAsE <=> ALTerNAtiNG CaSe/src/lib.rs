//! <https://www.codewars.com/kata/56efc695740d30f963000557/train/rust>

pub fn to_alternating_case(s: &str) -> String {
    // it's usually the same capacity
    let cap = s.len();
    let mut res = String::with_capacity(cap + 1);

    for c in s.chars() {
        if c.is_lowercase() {
            res.extend(c.to_uppercase())
        } else {
            res.extend(c.to_lowercase())
        }
    }

    res
}
