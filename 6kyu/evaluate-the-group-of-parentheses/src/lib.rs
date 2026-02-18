//! <https://www.codewars.com/kata/64b6722493f1050058dc3f98/train/rust>

pub fn eval_parentheses(s: &str) -> u32 {
    let mut res = 0;
    let mut d = 2;
    for i in 1..s.len() {
        if s.as_bytes()[i] == b'(' {
            d <<= 1;
        } else {
            d >>= 1;
            if s.as_bytes()[i - 1] != b')' {
                res += d;
            }
        }
    }
    res
}
