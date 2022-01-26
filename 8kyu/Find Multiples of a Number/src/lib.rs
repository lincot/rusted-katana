//! <https://www.codewars.com/kata/58ca658cc0d6401f2700045f/train/rust>

pub fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    (n..=limit).step_by(n as _).collect()
}
