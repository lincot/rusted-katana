//! <https://www.codewars.com/kata/55ab4f980f2d576c070000f4/train/rust>

pub fn game(n: u64) -> Vec<u64> {
    let x = n * n;
    if x % 2 == 0 {
        vec![x / 2]
    } else {
        vec![x, 2]
    }
}
