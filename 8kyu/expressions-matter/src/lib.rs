//! <https://www.codewars.com/kata/5ae62fcf252e66d44d00008e/train/rust>

pub const fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    if a != 1 && b != 1 && c != 1 {
        a * b * c
    } else if a == 1 && c == 1 {
        a + b + c
    } else if a < c {
        (a + b) * c
    } else {
        a * (b + c)
    }
}
