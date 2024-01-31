//! <https://www.codewars.com/kata/55f9b48403f6b87a7c0000bd/train/rust>

pub const fn paperwork(n: i16, m: i16) -> u32 {
    if n < 0 || m < 0 {
        0
    } else {
        (n * m) as _
    }
}
