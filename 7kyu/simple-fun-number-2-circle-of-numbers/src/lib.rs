//! <https://www.codewars.com/kata/58841cb52a077503c4000015/train/rust>

pub const fn circle_of_numbers(n: u32, fst: u32) -> u32 {
    if fst < n / 2 {
        fst + n / 2
    } else {
        fst - n / 2
    }
}
