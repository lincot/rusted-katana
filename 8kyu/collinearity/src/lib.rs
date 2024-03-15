//! <https://www.codewars.com/kata/65ba420888906c1f86e1e680/train/rust>

pub const fn collinearity(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    x1 * y2 == y1 * x2
}
