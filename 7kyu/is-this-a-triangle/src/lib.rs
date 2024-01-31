//! <https://www.codewars.com/kata/56606694ec01347ce800001b/train/rust>

pub const fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    a + b > c && b + c > a && a + c > b
}
