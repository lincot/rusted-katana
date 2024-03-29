//! <https://www.codewars.com/kata/64b7c3e6e0abed000f6cad6c/train/rust>

pub const fn block_player(a: u8, b: u8) -> u8 {
    match if a < b { [a, b] } else { [b, a] } {
        [0, 1] | [4, 6] | [5, 8] => 2,
        [0, 2] | [4, 7] => 1,
        [0, 3] | [2, 4] | [7, 8] => 6,
        [0, 4] | [2, 5] | [6, 7] => 8,
        [0, 6] | [4, 5] => 3,
        [0, 8] | [1, 7] | [2, 6] | [3, 5] => 4,
        [1, 4] | [6, 8] => 7,
        [2, 8] | [3, 4] => 5,
        _ => 0,
    }
}
