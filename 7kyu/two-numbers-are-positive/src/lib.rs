//! <https://www.codewars.com/kata/602db3215c22df000e8544f0/train/rust>

pub const fn two_are_positive(a: i32, b: i32, c: i32) -> bool {
    matches!(
        (a > 0, b > 0, c > 0),
        (false, true, true) | (true, false, true) | (true, true, false)
    )
}
