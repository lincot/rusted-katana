//! <https://www.codewars.com/kata/6405f2bb2894f600599172fd/train/rust>

pub const fn dots_on_domino_bones(n: i32) -> Option<u64> {
    let n = n as u32;
    Some((n * (n + 1) * (n + 2) / 2) as _)
}
