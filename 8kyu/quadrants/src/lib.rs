//! <https://www.codewars.com/kata/643af0fa9fa6c406b47c5399/train/rust>

pub const fn quadrant(x: i32, y: i32) -> i32 {
    match (x > 0, y > 0) {
        (true, true) => 1,
        (false, true) => 2,
        (false, false) => 3,
        (true, false) => 4,
    }
}
