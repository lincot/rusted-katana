//! <https://www.codewars.com/kata/5ce04eadd103f4001edd8986/train/rust>

pub fn solution(n: u8, b: u32) -> Vec<u32> {
    if b == 0 || b >= 1 << n {
        return vec![];
    }

    let right_mask = b - 1;
    let left_mask = !right_mask;

    (0..1 << (n - 1))
        .map(|x| ((x & left_mask) << 1) | b | (x & right_mask))
        .collect()
}
