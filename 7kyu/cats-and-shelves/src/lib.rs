//! <https://www.codewars.com/kata/62c93765cef6f10030dfa92b/train/rust>

pub const fn cats_and_shelves(start: u8, finish: u8) -> u8 {
    let jumps = finish - start;
    jumps / 3 + jumps % 3
}
