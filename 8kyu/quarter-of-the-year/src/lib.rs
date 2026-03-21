//! <https://www.codewars.com/kata/5ce9c1000bab0b001134f5af/train/rust>

pub const fn quarter_of(month: u8) -> u8 {
    match month {
        1..=3 => 1,
        4..=6 => 2,
        7..=9 => 3,
        10..=12 => 4,
        _ => panic!(),
    }
}
