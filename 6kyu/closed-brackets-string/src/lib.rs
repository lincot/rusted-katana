//! <https://www.codewars.com/kata/64b771989416793927fbd2bf/train/rust>

pub const fn closed_brackets(s: &str) -> bool {
    let s = s.as_bytes();
    let mut l = 0usize;
    let mut u = 0usize;
    let mut i = 0;
    while i < s.len() {
        let e = s[i];
        l = if e == b'(' {
            l + 1
        } else {
            l.saturating_sub(1)
        };
        if e != b')' {
            u += 1;
        } else if let Some(new_u) = u.checked_sub(1) {
            u = new_u;
        } else {
            return false;
        }
        i += 1;
    }
    l == 0
}
