//! <https://www.codewars.com/kata/5667e8f4e3f572a8f2000039/train/rust>

use std::iter::repeat;

pub fn accum(s: &str) -> String {
    let cap = if s.is_empty() {
        return String::new();
    } else {
        s.len() * ((s.len() + 1) / 2 + 1) - 1
    };
    let mut res = Vec::with_capacity(cap);

    let mut bytes = s.bytes();

    match bytes.next().unwrap() {
        b @ b'a'..=b'z' => {
            res.push(b - (b'a' - b'A'));
        }
        b @ b'A'..=b'Z' => {
            res.push(b);
        }
        _ => panic!(),
    }
    for (i, b) in (1..).zip(bytes) {
        res.push(b'-');
        match b {
            b'a'..=b'z' => {
                res.push(b - (b'a' - b'A'));
                res.extend(repeat(b).take(i));
            }
            b'A'..=b'Z' => {
                res.push(b);
                res.extend(repeat(b + (b'a' - b'A')).take(i));
            }
            _ => panic!(),
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}
