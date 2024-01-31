//! <https://www.codewars.com/kata/525e5a1cb735154b320002c8/train/rust>

pub const fn triangular(n: i32) -> i32 {
    if n > 0 {
        n * (n + 1) / 2
    } else {
        0
    }
}
