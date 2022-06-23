//! <https://www.codewars.com/kata/588e68aed4cff457d300002e/train/rust>

pub fn turn(current: char, target: char) -> String {
    match (current, target) {
        ('N', 'W') | ('W', 'S') | ('S', 'E') | ('E', 'N') => "left",
        _ => "right",
    }
    .into()
}
