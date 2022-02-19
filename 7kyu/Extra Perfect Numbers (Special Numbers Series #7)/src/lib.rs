//! <https://www.codewars.com/kata/5a662a02e626c54e87000123/train/rust>

pub fn extra_perfect(n: u32) -> Vec<u32> {
    (1..=n).step_by(2).collect()
}
