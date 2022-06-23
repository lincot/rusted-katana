//! <https://www.codewars.com/kata/5a145ab08ba9148dd6000094/train/rust>

pub fn doubles(s: &str) -> String {
    // worst care capacity
    let mut res = String::with_capacity(s.len());

    for c in s.chars() {
        if res.ends_with(c) {
            res.pop();
        } else {
            res.push(c);
        }
    }

    res
}
