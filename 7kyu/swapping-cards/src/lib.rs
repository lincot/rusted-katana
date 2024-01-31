//! <https://www.codewars.com/kata/65127302a5de2b11c940973d/train/rust>

pub fn swap_cards(n1: u8, n2: u8) -> bool {
    let (a, b) = (n1 / 10, n1 % 10);
    let (x, y) = (n2 / 10, n2 % 10);
    b < a || (a, y) < (x, b)
}
