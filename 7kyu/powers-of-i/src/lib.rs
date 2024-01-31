//! <https://www.codewars.com/kata/5a97387e5ee396e70a00016d/train/rust>

pub const fn pofi(n: u32) -> &'static str {
    match n % 4 {
        0 => "1",
        1 => "i",
        2 => "-1",
        3 => "-i",
        _ => unreachable!(),
    }
}
