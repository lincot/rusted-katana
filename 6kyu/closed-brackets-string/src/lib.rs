//! <https://www.codewars.com/kata/64b771989416793927fbd2bf/train/rust>

pub fn closed_brackets(s: &str) -> bool {
    let mut a = 0isize;
    let mut b = 0isize;
    for e in s.bytes() {
        if e == b'(' {
            a += 1;
            b += 1;
        } else if e == b')' {
            a = (a - 1).max(0);
            b -= 1;
            if b < 0 {
                break;
            }
        } else {
            a = (a - 1).max(0);
            b += 1;
        }
    }
    a <= 0 && b >= 0
}
