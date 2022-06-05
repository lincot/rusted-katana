//! <https://www.codewars.com/kata/56efc695740d30f963000557/train/rust>

pub fn to_alternating_case(s: &str) -> String {
    // usually the same capacity
    let mut res = String::with_capacity(s.len());

    for c in s.chars() {
        if c.is_lowercase() {
            res.extend(c.to_uppercase());
        } else {
            res.extend(c.to_lowercase());
        }
    }

    res
}
