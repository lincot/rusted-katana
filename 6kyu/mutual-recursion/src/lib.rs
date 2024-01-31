//! <https://www.codewars.com/kata/53a1eac7e0afd3ad3300008b/train/rust>

pub fn f(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n - m(f(n - 1))
    }
}

pub fn m(n: i32) -> i32 {
    if n == 0 {
        0
    } else {
        n - f(m(n - 1))
    }
}
