//! <https://www.codewars.com/kata/59831e3575ca6c8aea00003a/train/rust>

pub const fn game(n: u32, m: u32) -> &'static str {
    if n % 2 == 1 && m != 2 {
        "first"
    } else {
        "second"
    }
}
