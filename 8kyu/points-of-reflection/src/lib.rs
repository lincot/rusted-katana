//! <https://www.codewars.com/kata/57bfea4cb19505912900012c/train/rust>

pub const fn symmetric_point(p: [i32; 2], q: [i32; 2]) -> [i32; 2] {
    [2 * q[0] - p[0], 2 * q[1] - p[1]]
}
