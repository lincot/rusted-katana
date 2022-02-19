//! <https://www.codewars.com/kata/5dd259444228280032b1ed2a/train/rust>

pub const fn solve(sum: u32, gcd: u32) -> Option<(u32, u32)> {
    if sum % gcd == 0 {
        Some((gcd, sum - gcd))
    } else {
        None
    }
}
